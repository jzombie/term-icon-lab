# term-icon

**NOTE: THIS IS ENTIRELY ALPHA**

Single-cell Unicode icons for terminal UIs — **empirically verified**, not
assumed. Every glyph in this crate has been proven, by automated
screenshot-and-PTY analysis on real terminals, to occupy exactly **one
character cell (1×1)** with no bleed into neighbors, simultaneously on:

| Platform | Terminal | Font |
|---|---|---|
| Linux (ubuntu runner) | `xterm` | DejaVu Sans Mono |
| macOS (macos runner) | Terminal.app | stock SF Mono/Menlo |
| Windows (windows runner) | `conhost` | Consolas |

The verification matrix runs on every push. A glyph that renders 2 cells
wide, paints into its neighbor, or doesn't render at all on **any** of the
three is permanently excluded. What ships is what a stock, unmodified OS
terminal can actually draw — no Nerd Fonts, no patched fonts, no PUA
codepoints.

## Usage

```toml
[dependencies]
term-icon = "0.1"
```

```rust
use term_icon::universal;

// Every verified icon (currently 1000+), sorted by codepoint.
for icon in universal::UNIVERSAL_ICONS {
    println!("{}", icon); // writes the glyph
}

// Look up by catalog id or by character.
let vertical = universal::lookup("box_2502").unwrap();     // │
let also = universal::by_codepoint('│').unwrap();

// Search by Unicode name, id, block, codepoint (hex/decimal) or glyph.
for entry in universal::search("star") {
    println!(
        "{} {} U+{:04X} {}",
        entry.icon.glyph, entry.id, entry.codepoint, entry.unicode_name
    );
}

// Icons are grouped per Unicode block.
let gear_like = &universal::misc_symbols::MISC_2699;

// Every icon carries a conservative ASCII fallback for degraded
// environments (logging, non-Unicode sinks, missing font coverage).
let safe = vertical.fallback_or_glyph(); // "|" when the glyph can't be used
```

### ratatui (optional feature)

```toml
[dependencies]
term-icon = { version = "0.1", features = ["ratatui"] }
```

```rust
use term_icon::ratatui_ext::SetSafeIcon;

// Writes the glyph into one buffer cell (bounds-clipped, never panics)
// and advances the cursor by exactly 1 column.
let next_x = buffer.set_safe_icon(x, y, &icon, style);
```

## Browsing the set

The repo ships a CLI for exploring what's available:

```sh
cargo install --path crates/icons   # provides the `icons` command

icons gallery                        # all verified glyphs, grouped by block
icons gallery --block braille        # one block only
icons search star                    # by Unicode name, id, block,
icons search 0x2502                  # codepoint (hex/decimal),
icons search │                       # or the glyph itself
icons show box_2502                  # full metadata for one icon
icons export GALLERY.md              # deterministic markdown cheat-sheet
```

[`GALLERY.md`](GALLERY.md) is the committed, human-browsable version of the
entire verified set. Everything listed there is guaranteed by the latest CI
verification run (see below for the exact meaning of "guaranteed").

## How verification works

The workspace is a measurement pipeline disguised as a Rust build:

```
catalog (1165 candidates: ASCII, Arrows, Box Drawing, Block Elements,
         Geometric Shapes, Misc Symbols, Dingbats, Braille)
   │
   ▼  matrix-harness — renders pages of candidates inside a REAL terminal
   │  (PTY), one 24-row page at a time, synced to the screenshot
   │  orchestrator via .ready/.ack marker files.
   │  Pass 1: cursor-position reports prove the glyph advances exactly
   │  one column (a 2-cell render reports column 9, not 8) and correlate
   │  every row's reported position against the page layout.
   │
   ▼  capture — one screenshot per page per platform
   │  (linux: Xvfb + xterm + ImageMagick, macos: Terminal.app +
   │  screencapture, windows: conhost + GDI), each frame gated by an
   │  entropy/plausibility check with one retry.
   │
   ▼  pixel-assert — Pass 2 over the pixels:
   │  * visibility: the glyph's cell contains real ink (block-aware
   │    thresholds; tofu-box detection with rectangularity check)
   │  * no bleed: no ink past the cell boundary beyond anti-aliasing
   │    tolerance; sentinel B's cell pixel-identical to the control row
   │
   ▼  manifest-gen — THE AND GATE: keep only glyphs that passed every
      pass on every platform, resolve their official Unicode names from
      a vendored UnicodeData extract, and emit src/generated_manifest.rs.
      Exit hysteresis keeps the set stable: an already-verified icon is
      dropped only if it fails on 2+ platforms in a single run (one
      runner's subpixel rendering difference can't purge it).
```

CI (`.github/workflows/matrix.yml`) runs the full loop on all three
platforms, then a **drift check**: the committed
`src/generated_manifest.rs` must be byte-identical to a fresh regeneration.
If the verified set changed, CI fails with instructions to regenerate and
commit — the manifest can never silently claim more (or less) than the last
run proved.

### What "verified" means — and doesn't

- **Spatial guarantee**: the glyph advances exactly 1 column and paints
  nowhere outside its cell (beyond anti-aliasing). It says nothing about
  style — a symbol that renders as a color emoji but stays 1×1 passes.
- **Per terminal+font**: the three CI defaults above are the authority.
  Other terminals (Windows Terminal, kitty, Alacritty, …) and user-installed
  fonts are untested; stock runner fonts are the floor.
- **As of the latest run**: the set is re-measured on every push. Icons that
  regress are removed; `fallback_or_glyph()` gives you a conservative ASCII
  substitute regardless.

## Development

```
crates/catalog        candidate registry: which codepoints enter verification
crates/matrix-harness Pass 1: renders pages in a PTY, DSR cursor tracking
crates/pixel-assert   Pass 2: screenshot geometry/visibility/bleed asserts
crates/manifest-gen   AND gate + codegen for src/generated_manifest.rs
crates/icons          gallery/search/show/export CLI
scripts/              per-platform capture orchestrators + UCD updater
```

Common tasks:

```sh
cargo test --workspace                # everything, no terminal needed
cargo run -p matrix-harness -- --self-check   # catalog/pagination smoke test
scripts/local-e2e.sh                  # full linux capture + assert locally
scripts/update-ucd.sh                 # refresh vendored Unicode names
```

To expand what gets verified, add ranges in `crates/catalog/src/lib.rs`
(and the matching block/prefix/fallback arms) and push — CI verifies the new
candidates and the drift check walks you through adopting the survivors.
Vendored Unicode names: `scripts/update-ucd.sh` re-extracts
`crates/manifest-gen/ucd/UnicodeData.txt` when ranges change.

## License / scope notes

- The `ratatui` feature is the only integration; the core crate has **zero
  dependencies**.
- Out of scope by design: PUA/Nerd-Font glyphs, 2-cell (wide) icons, and any
  glyph requiring a patched font.
