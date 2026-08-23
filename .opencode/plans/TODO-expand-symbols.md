Box drawing lines, block fills, and braille dots are structural TUI layout primitives and graph/spinner components, not UI icons.

This mismatch happened because purging the `nerd` feature (Nerd Fonts / PUA codepoints) eliminated the entire catalog of folder, file, brand, and git logos. Strictly requiring zero user font installation and zero grid drift collapsed the remaining catalog down to standard monospace text primitives.

If `term-icon` is going to live up to its name without requiring third-party patched fonts, the catalog must be expanded to include standard Unicode symbol blocks that exist in stock system fonts (Cascadia Code, SF Mono, DejaVu Sans Mono).

### Unicode Blocks That Contain Actual Icons

Adding these standard Unicode ranges to `crates/catalog` introduces real UI icons that do not require Nerd Fonts and still satisfy $1 \times 1$ grid constraints:

* **Miscellaneous Symbols (`U+2600–U+26FF`):** Status and UI icons — gear (`⚙`), warning triangle (`⚠`), star (`★`), flag (`⚑`), checkmark (`✔`), cross (`✖`), cloud (`☁`), lightning (`⚡`).
* **Geometric Shapes (`U+25A0–U+25FF`):** UI indicators — status dots (`●`, `◯`), play/expand arrows (`▲`, `▼`, `▶`, `◀`), diamonds (`◆`), selection boxes (`■`, `□`).
* **Dingbats (`U+2700–U+27BF`):** Action icons — prompt arrows (`➜`, `➔`), edit pencil (`✎`), heavy check/cross (`✔`, `✖`), scissors (`✂`).
* **Arrows (`U+2190–U+21FF`):** Navigation icons — direction arrows (`←`, `↑`, `→`, `↓`), refresh/cycle arrows (`↺`, `↻`), swap arrows (`↔`).

### Action Plan

1. **Add Symbol Ranges to `crates/catalog`:** Update candidate generation to include `0x2190..=0x21FF`, `0x25A0..=0x25FF`, `0x2600..=0x26FF`, and `0x2700..=0x27BF`.
2. **Let the CI Filter Run:** The Pass 1 / Pass 2 pipeline will test these symbol candidates across macOS, Windows, and Linux. Any symbol that expands to 2 cells, fails font coverage, or bleeds on stock runners will be automatically dropped by `manifest-gen`.
3. **Result:** `UNIVERSAL_ICONS` gets a verified set of real UI icons (stars, gears, warnings, arrows, checkmarks) that work out-of-the-box on stock operating system terminals.
