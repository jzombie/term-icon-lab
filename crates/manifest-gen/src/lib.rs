//! Shared parsers from `manifest-gen`, exposed for sibling pipeline tools.
//!
//! Only generic, side-effect-free file parsing lives here. The AND gate and
//! exit hysteresis remain private to the `manifest-gen` binary — it is the
//! sole authority on the shipped icon set.

use std::collections::{HashMap, HashSet};
use std::path::Path;

use anyhow::Context;

/// Ids present in a previously committed (or freshly regenerated) generated
/// manifest. A missing file yields an empty set, mirroring the bin's
/// `--previous` semantics.
#[must_use]
pub fn previous_ids(path: &Path) -> HashSet<String> {
    let raw = match std::fs::read_to_string(path) {
        Ok(raw) => raw,
        Err(_) => return HashSet::new(),
    };
    let mut ids = HashSet::new();
    for line in raw.lines() {
        let Some(pos) = line.find("id: \"") else {
            continue;
        };
        let rest = &line[pos + 5..];
        let Some(end) = rest.find('"') else { continue };
        ids.insert(rest[..end].to_string());
    }
    ids
}

/// Load the vendored UCD extract into a codepoint → official-name map.
///
/// Parsing is bounds-checked: fields are pulled from the `;`-split iterator
/// by `next()`, never by slice indexing — malformed or truncated lines are
/// skipped with a counted warning rather than panicking.
pub fn load_ucd_names(path: &Path) -> Result<HashMap<u32, String>, anyhow::Error> {
    let raw = std::fs::read_to_string(path)
        .with_context(|| format!("read UCD extract {}", path.display()))?;
    let mut names = HashMap::new();
    let mut skipped = 0usize;
    for line in raw.lines() {
        let mut fields = line.split(';');
        let (Some(raw_cp), Some(name)) = (fields.next(), fields.next()) else {
            skipped += 1;
            continue;
        };
        match u32::from_str_radix(raw_cp, 16) {
            Ok(cp) => {
                names.insert(cp, name.to_string());
            }
            Err(_) => skipped += 1,
        }
    }
    if skipped > 0 {
        eprintln!(
            "warning: skipped {skipped} malformed UCD lines in {}",
            path.display()
        );
    }
    Ok(names)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scrapes_ids_from_generated_manifest_lines() {
        let dir = std::env::temp_dir().join(format!("ti_mg_lib_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("generated_manifest.rs");
        std::fs::write(
            &path,
            r#"
pub const BOX_2502: IconEntry = IconEntry { id: "box_2502", codepoint: 0x2502, .. };
pub const MISC_2605: IconEntry = IconEntry { id: "misc_2605", codepoint: 0x2605, .. };
"#,
        )
        .unwrap();

        let ids = previous_ids(&path);
        assert!(ids.contains("box_2502"));
        assert!(ids.contains("misc_2605"));

        // Missing file ⇒ empty set (bootstrap-friendly).
        assert!(previous_ids(&dir.join("nope.rs")).is_empty());
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn ucd_parse_is_lenient_and_resolves_names() {
        let dir = std::env::temp_dir().join(format!("ti_mg_ucd_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("UnicodeData.txt");
        std::fs::write(
            &path,
            "2502;BOX DRAWINGS LIGHT VERTICAL;Sm;0;ON;;;;;N;;;;;\ngarbage-line\nZZZZ;BAD CP;\n",
        )
        .unwrap();

        let names = load_ucd_names(&path).unwrap();
        assert_eq!(
            names.get(&0x2502).map(String::as_str),
            Some("BOX DRAWINGS LIGHT VERTICAL")
        );
        assert_eq!(names.len(), 1, "malformed lines are skipped");
        let _ = std::fs::remove_dir_all(&dir);
    }
}
