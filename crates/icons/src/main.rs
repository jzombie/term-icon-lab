//! icons: browse and search the term-icon verified glyph set.
//!
//! Subcommands:
//! * `icons gallery [--block <name>]` — visual table of every verified glyph.
//! * `icons search <query>` — case-insensitive match over Unicode name, id,
//!   block, codepoint (hex/decimal), or the glyph itself.
//! * `icons show <id-or-codepoint>` — detailed metadata for one icon.
//! * `icons export [path]` — deterministic Markdown cheat-sheet
//!   (`-` or omitted = stdout).
//!
//! Exit codes: `0` success, `1` no matches, `2` usage error.

use std::io::Write;
use std::path::PathBuf;

use clap::{Parser, Subcommand};
use term_icon::universal::{self, IconEntry};

#[derive(Parser)]
#[command(
    name = "icons",
    about = "Browse and search the term-icon verified glyph set.",
    version
)]
struct Cli {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(Subcommand)]
enum Cmd {
    /// Render every verified glyph, grouped by Unicode block.
    Gallery {
        /// Restrict to one block (e.g. `arrows`, `box_drawing`, `braille`).
        #[arg(long)]
        block: Option<String>,
    },
    /// Case-insensitive search over Unicode name, id, block, codepoint, glyph.
    Search { query: String },
    /// Show detailed metadata for one icon by catalog id or codepoint.
    Show { id_or_codepoint: String },
    /// Write a deterministic Markdown cheat-sheet (path or stdout).
    Export { path: Option<PathBuf> },
}

fn main() -> std::process::ExitCode {
    let cli = Cli::parse();
    let code = match cli.cmd {
        Cmd::Gallery { block } => gallery(block.as_deref()),
        Cmd::Search { query } => search(&query),
        Cmd::Show { id_or_codepoint } => show(&id_or_codepoint),
        Cmd::Export { path } => export(path.as_deref()),
    };
    match code {
        0 => std::process::ExitCode::SUCCESS,
        1 => std::process::ExitCode::from(1),
        _ => std::process::ExitCode::from(2),
    }
}

fn header() {
    println!(
        "{:<4} {:<16} {:<8} {:<16} {:<44} FALLBACK",
        "GLYPH", "ID", "CODEPOINT", "BLOCK", "UNICODE NAME"
    );
    println!("{}", "-".repeat(96));
}

fn row(e: &IconEntry) {
    println!(
        "{:<4} {:<16} U+{:04X}  {:<16} {:<44} {}",
        e.icon.glyph,
        e.id,
        e.codepoint,
        e.block,
        e.unicode_name,
        e.icon.fallback_or_glyph()
    );
}

fn gallery(block: Option<&str>) -> i32 {
    let wanted = block.map(|b| b.to_ascii_lowercase());
    let mut shown = 0usize;
    let mut last_block = String::new();
    for e in universal::ICON_ENTRIES {
        if let Some(w) = &wanted
            && e.block.to_ascii_lowercase() != *w
        {
            continue;
        }
        if e.block != last_block {
            println!("== {} ==", e.block);
            header();
            last_block = e.block.to_string();
        }
        row(e);
        shown += 1;
    }
    if shown == 0 {
        eprintln!("icons: no matches");
        return 1;
    }
    println!("\n{} icons", shown);
    0
}

fn search(query: &str) -> i32 {
    header();
    let mut shown = 0usize;
    for e in universal::search(query) {
        row(e);
        shown += 1;
    }
    if shown == 0 {
        eprintln!("icons: no matches for {query:?}");
        return 1;
    }
    println!("\n{} icons", shown);
    0
}

/// Resolve a `show` argument: `0x…`/`U+…` hex, decimal, or a catalog id.
fn resolve(arg: &str) -> Option<&'static IconEntry> {
    let t = arg.trim();
    if let Some(hex) = t.strip_prefix("0x").or_else(|| t.strip_prefix("U+"))
        && let Ok(cp) = u32::from_str_radix(hex, 16)
    {
        return universal::ICON_ENTRIES.iter().find(|e| e.codepoint == cp);
    }
    if let Ok(cp) = t.parse::<u32>() {
        return universal::ICON_ENTRIES.iter().find(|e| e.codepoint == cp);
    }
    universal::ICON_ENTRIES.iter().find(|e| e.id == t)
}

fn show(arg: &str) -> i32 {
    let Some(e) = resolve(arg) else {
        eprintln!("icons: no icon for {arg:?}");
        return 1;
    };
    println!("glyph      {}", e.icon.glyph);
    println!("id         {}", e.id);
    println!("const      {}::{}", e.block, e.id.to_ascii_uppercase());
    println!("codepoint  U+{:04X} ({})", e.codepoint, e.codepoint);
    println!("block      {}", e.block);
    println!("unicode name {}", e.unicode_name);
    println!("fallback   {}", e.icon.fallback_or_glyph());
    0
}

/// Deterministic Markdown cheat-sheet: one `## <block>` section per block in
/// codepoint order.
fn markdown(entries: impl Iterator<Item = &'static IconEntry>) -> String {
    let mut out = String::from("# term-icon verified gallery\n\n");
    out.push_str(
        "Every glyph below passed dual-pass verification (PTY + pixel) on \
macOS Terminal.app, Windows conhost/WT, and Linux xterm simultaneously.\n\n",
    );
    let mut last_block = String::new();
    for e in entries {
        if e.block != last_block {
            if !last_block.is_empty() {
                out.push('\n');
            }
            out.push_str(&format!("## {}\n\n", e.block));
            out.push_str("| Glyph | Id | Codepoint | Unicode name | Fallback |\n");
            out.push_str("|---|---|---|---|---|\n");
            last_block = e.block.to_string();
        }
        out.push_str(&format!(
            "| {} | `{}` | U+{:04X} | {} | `{}` |\n",
            e.icon.glyph,
            e.id,
            e.codepoint,
            if e.unicode_name.is_empty() {
                "—"
            } else {
                e.unicode_name
            },
            e.icon.fallback_or_glyph()
        ));
    }
    out
}

fn export(path: Option<&std::path::Path>) -> i32 {
    let doc = markdown(universal::ICON_ENTRIES.iter());
    match path {
        Some(p) => {
            if let Err(err) = std::fs::write(p, &doc) {
                eprintln!("icons: cannot write {}: {err}", p.display());
                return 2;
            }
            println!("wrote {} ({} bytes)", p.display(), doc.len());
            0
        }
        None => {
            let mut stdout = std::io::stdout();
            let _ = stdout.write_all(doc.as_bytes());
            0
        }
    }
}
