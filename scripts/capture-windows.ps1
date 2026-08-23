#requires -version 5.1
<#
.SYNOPSIS
  Windows capture orchestrator for the term-icon verification matrix.
.DESCRIPTION
  Routing contract (no display-driver installs anywhere):
    * Default backend: run matrix-harness under explicit conhost.exe and
      capture with GDI (PrintWindow/BitBlt against the console host window),
      which executes deterministically against the Win32 subsystem. The run is
      tagged host="conhost" so manifest-gen knows the emulator differed.
    * Opt-in experimental (TERM_ICON_WT_EXPERIMENTAL=1): when a native display
      is present, launch wt.exe and capture its DWM-COMPOSED surface via
      PrintWindow(PW_RENDERFULLCONTENT) — this reads the composited frame
      rather than poking the DirectX swapchain. Any capture-validation failure
      automatically falls back to the conhost backend so CI stays actionable.
      Promote to default once validated on a real runner.
  HWND acquisition uses a deterministic poll loop: wt.exe is an async
  bootstrapper whose MainWindowHandle stays IntPtr.Zero until WinUI finishes
  window creation. Every `.ready` poll iteration checks harness-PID liveness
  so fail-fast exits abort immediately.
#>
[CmdletBinding()]
param(
    [string]$OutDir = "target/matrix",
    [int]$HwndTimeoutMs = 10000
)
$ErrorActionPreference = "Stop"

# Load GDI+ before any Add-Type compilation so C# blocks referencing
# System.Drawing resolve their assembly references.
Add-Type -AssemblyName System.Drawing

$Repo = (Get-Location).Path
$Harness = Join-Path $Repo "target\debug\matrix-harness.exe"
$PixelAssert = Join-Path $Repo "target\debug\pixel-assert.exe"
if (-not (Test-Path $Harness)) { throw "matrix-harness.exe not built" }

New-Item -ItemType Directory -Force -Path "$OutDir\pages", "$OutDir\artifacts" | Out-Null
$SyncDir = Join-Path ([System.IO.Path]::GetTempPath()) ("term_icon_" + [Guid]::NewGuid().ToString("N"))
New-Item -ItemType Directory -Force -Path $SyncDir | Out-Null

# Windows PowerShell 5.1 compiles with a fixed default reference set that
# excludes System.Drawing.dll; loaded assemblies are not compile references.
Add-Type -ReferencedAssemblies System.Drawing @"
using System;
using System.Drawing;
using System.Drawing.Imaging;
using System.Runtime.InteropServices;
public static class TermShot {
    [StructLayout(LayoutKind.Sequential)] struct RECT { public int L, T, R, B; }
    [DllImport("user32.dll")] static extern bool GetWindowRect(IntPtr h, out RECT r);
    [DllImport("user32.dll")] static extern bool PrintWindow(IntPtr h, IntPtr dc, uint flags);

    // PW_RENDERFULLCONTENT (2): captures the DWM-composed surface, including
    // DirectWrite/DirectX text rendering — required for wt.exe; harmless for
    // the GDI-rendered conhost window.
    public static bool Shot(IntPtr hwnd, string path) {
        RECT r;
        if (!GetWindowRect(hwnd, out r)) return false;
        int w = r.R - r.L, h = r.B - r.T;
        if (w <= 0 || h <= 0) return false;
        Bitmap bmp = null;
        try {
            bmp = new Bitmap(w, h, PixelFormat.Format32bppArgb);
            using (Graphics g = Graphics.FromImage(bmp)) {
                IntPtr dc = g.GetHdc();
                bool ok = PrintWindow(hwnd, dc, 2);
                g.ReleaseHdc(dc);
                if (!ok) return false;
            }
            bmp.Save(path, ImageFormat.Png);
            return System.IO.File.Exists(path);
        } finally { if (bmp != null) bmp.Dispose(); }
    }
}
"@

function Test-NativeDisplay {
    Add-Type -TypeDefinition @"
using System;
using System.Runtime.InteropServices;
public static class DispDev {
    [StructLayout(LayoutKind.Sequential, CharSet = CharSet.Ansi)]
    public struct DISPLAY_DEVICE {
        public int cb;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst=32)] public string DeviceName;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst=128)] public string DeviceString;
        public int StateFlags;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst=128)] public string DeviceID;
        [MarshalAs(UnmanagedType.ByValTStr, SizeConst=128)] public string DeviceKey;
    }
    [DllImport("user32.dll", CharSet = CharSet.Ansi)]
    public static extern bool EnumDisplayDevices(string dev, int num, ref DISPLAY_DEVICE dd, int flags);
    public static bool HasActiveDisplay() {
        DISPLAY_DEVICE dd = new DISPLAY_DEVICE(); dd.cb = Marshal.SizeOf(dd);
        for (int i = 0; EnumDisplayDevices(null, i, ref dd, 0); i++) {
            if ((dd.StateFlags & 1) != 0) return true; // ATTACHED_TO_DESKTOP
            dd.cb = Marshal.SizeOf(dd);
        }
        return false;
    }
}
"@
    return [DispDev]::HasActiveDisplay()
}

function Resolve-Hwnd {
    # Primary: the process we spawned owns its console/window; poll its own
    # MainWindowHandle (wt.exe may forward to an existing instance and exit —
    # then the name-scan fallback below takes over).
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    while ($sw.ElapsedMilliseconds -lt $HwndTimeoutMs) {
        if ($script:TermProc) {
            $script:TermProc.Refresh()
            if (-not $script:TermProc.HasExited -and
                $script:TermProc.MainWindowHandle -ne [IntPtr]::Zero) {
                return $script:TermProc.MainWindowHandle
            }
        }
        Start-Sleep -Milliseconds 100
    }

    # Fallback: newest windowed terminal-family process.
    $sw.Restart()
    while ($sw.ElapsedMilliseconds -lt $HwndTimeoutMs) {
        $proc = Get-Process -Name "WindowsTerminal","OpenConsole","conhost","powershell","pwsh" -ErrorAction SilentlyContinue |
            Where-Object { $_.MainWindowHandle -ne [IntPtr]::Zero } |
            Sort-Object StartTime -Descending | Select-Object -First 1
        if ($proc) { return $proc.MainWindowHandle }
        Start-Sleep -Milliseconds 100
    }
    throw "Timed out waiting for terminal HWND initialization."
}

function Start-HarnessRun {
    param([bool]$ViaConhost)
    $inner = Join-Path $SyncDir "run.ps1"
    $hostName = if ($ViaConhost) { 'conhost' } else { 'wt' }
    # The wrapper publishes its own PID (the main script's liveness anchor),
    # keeps the harness stdout on the console — it renders there and waits
    # for CPR replies from the terminal host — and redirects only stderr.
    # -RedirectStandardError is a raw handle redirect at process creation;
    # PS 5.1's `2>` operator routes native stderr through error records and
    # is unreliable under $ErrorActionPreference = 'Stop'.
    @"
Set-Content -Path '$SyncDir\pid' -Value `$PID
`$p = Start-Process -FilePath '$Harness' -ArgumentList @(
    '--sync-dir', '$SyncDir',
    '--out-dir', '$OutDir\artifacts',
    '--platform', 'windows/$hostName',
    '--host', '$hostName'
) -RedirectStandardError '$SyncDir\harness.log' -NoNewWindow -PassThru -Wait
exit `$p.ExitCode
"@ | Set-Content -Path $inner -Encoding UTF8

    if ($ViaConhost) {
        $script:TermProc = Start-Process conhost.exe -PassThru -ArgumentList "powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "`"$inner`""
    } else {
        $script:TermProc = Start-Process wt.exe -PassThru -ArgumentList "-w", "_new", "powershell", "-NoProfile", "-ExecutionPolicy", "Bypass", "-File", "`"$inner`""
    }

    # PID marker published by the harness wrapper.
    $sw = [System.Diagnostics.Stopwatch]::StartNew()
    while (-not (Test-Path "$SyncDir\pid")) {
        if ($sw.ElapsedSeconds -gt 15) { throw "harness never published its PID." }
        Start-Sleep -Milliseconds 100
    }
    return [int](Get-Content "$SyncDir\pid")
}

$wantWt = -not $env:TERM_ICON_CONHOST_FORCE
if ($env:TERM_ICON_WT_EXPERIMENTAL -ne "1") { $wantWt = $false }
if ($wantWt -and -not (Test-NativeDisplay)) { $wantWt = $false }

$harnessPid = Start-HarnessRun -ViaConhost:(-not $wantWt)
$hwnd = Resolve-Hwnd

function Invoke-Capture {
    param([int]$Page)
    $shot = "$OutDir\pages\shot_page_$Page.png"
    if (-not [TermShot]::Shot([IntPtr]$hwnd, $shot)) { return $false }
    & $PixelAssert --validate-only $shot | Out-Null
    return ($LASTEXITCODE -eq 0)
}

# Page loop: capture every signaled page until the harness exits.
$page = 0
$conhostRetried = $false
while ($true) {
    $ready = Get-ChildItem $SyncDir -Filter "*_page_${page}.ready" -ErrorAction SilentlyContinue |
        Select-Object -First 1
    if ($ready) {
        if (-not (Invoke-Capture -Page $page)) {
            Start-Sleep -Milliseconds 800
            if (-not (Invoke-Capture -Page $page)) {
                if ($wantWt -and -not $conhostRetried) {
                    Write-Warning "wt capture backend failed validation; falling back to conhost."
                    $conhostRetried = $true
                    Stop-Process -Id $harnessPid -Force -ErrorAction SilentlyContinue
                    Get-ChildItem $SyncDir -Filter "*.ready" | Remove-Item -Force -ErrorAction SilentlyContinue
                    Get-ChildItem $SyncDir -Filter "*.ack" | Remove-Item -Force -ErrorAction SilentlyContinue
                    Remove-Item "$OutDir\pages\*.png" -Force -ErrorAction SilentlyContinue
                    $harnessPid = Start-HarnessRun -ViaConhost:$true
                    $hwnd = Resolve-Hwnd
                    $page = 0
                    continue
                }
                throw "capture validation failed twice for page $page"
            }
        }
        $ack = $ready.FullName -replace '\.ready$', '.ack'
        Set-Content -Path $ack -Value "ack"
        $page++
        continue
    }
    if (-not (Get-Process -Id $harnessPid -ErrorAction SilentlyContinue)) { break }
    Start-Sleep -Milliseconds 50
}

Wait-Process -Id $harnessPid -ErrorAction SilentlyContinue

if (-not (Test-Path "$OutDir\artifacts\sidecar.json")) {
    Write-Error "harness finished but sidecar.json is missing; harness.log tail:"
    if (Test-Path "$SyncDir\harness.log") { Get-Content "$SyncDir\harness.log" -Tail 40 | Write-Error }
    exit 2
}

$pagesCaptured = @(Get-ChildItem "$OutDir\pages" -Filter "shot_page_*.png" -ErrorAction SilentlyContinue).Count
if ($pagesCaptured -eq 0) {
    Write-Error "no pages were captured; sync dir contents:"
    Get-ChildItem $SyncDir -ErrorAction SilentlyContinue |
        ForEach-Object { Write-Error ("  {0} ({1} bytes)" -f $_.Name, $_.Length) }
    if (Test-Path "$SyncDir\harness.log") {
        Write-Error "harness.log tail:"
        Get-Content "$SyncDir\harness.log" -Tail 40 | ForEach-Object { Write-Error $_ }
    }
    exit 2
}

# Pass 2 over all captured pages (ordered).
$pngArgs = @()
Get-ChildItem "$OutDir\pages" -Filter "shot_page_*.png" |
    Sort-Object { [int]($_.BaseName -replace '^shot_page_', '') } |
    ForEach-Object { $pngArgs += @("--png", $_.FullName) }

& $PixelAssert @pngArgs `
    --sidecar "$OutDir\artifacts\sidecar.json" `
    --pass1 "$OutDir\artifacts\pass1.json" `
    --out "$OutDir\verdicts.json"
# Exit 1 = legitimate per-glyph verification failures — the verdicts are the
# deliverable; only usage/internal errors (>= 2) fail the run.
if ($LASTEXITCODE -ge 2) { exit 1 }

Remove-Item -Recurse -Force $SyncDir -ErrorAction SilentlyContinue

# powershell -File propagates the last native exit code; end explicitly so a
# verification-failure exit (1) from pixel-assert does not fail the job.
exit 0
