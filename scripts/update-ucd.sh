#!/usr/bin/env bash
# Vendor official Unicode character names for the catalog ranges.
#
# Downloads UnicodeData.txt (latest UCD) and keeps only the codepoints the
# catalog enumerates, writing a filtered extract to
# crates/manifest-gen/ucd/UnicodeData.txt. Re-run only when catalog ranges
# change in crates/catalog/src/lib.rs.
#
# Usage: scripts/update-ucd.sh [SOURCE_URL_OR_FILE]
set -euo pipefail

REPO_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
OUT="$REPO_ROOT/crates/manifest-gen/ucd/UnicodeData.txt"
SRC="${1:-https://www.unicode.org/Public/UCD/latest/ucd/UnicodeData.txt}"

mkdir -p "$(dirname "$OUT")"

python3 - "$SRC" "$OUT" <<'PY'
import sys, urllib.request

src, out = sys.argv[1], sys.argv[2]
if src.startswith("http://") or src.startswith("https://"):
    data = urllib.request.urlopen(src).read().decode()
else:
    with open(src) as f:
        data = f.read()

ranges = [
    (0x21, 0x7E),      # Basic ASCII
    (0x2190, 0x21FF),  # Arrows
    (0x2500, 0x257F),  # Box Drawing
    (0x2580, 0x259F),  # Block Elements
    (0x25A0, 0x25FF),  # Geometric Shapes
    (0x2600, 0x26FF),  # Miscellaneous Symbols
    (0x2700, 0x27BF),  # Dingbats
    (0x2801, 0x28FF),  # Braille Patterns
]

kept = []
for line in data.splitlines():
    if not line:
        continue
    try:
        cp = int(line.split(";", 1)[0], 16)
    except ValueError:
        continue
    if any(lo <= cp <= hi for lo, hi in ranges):
        kept.append(line)

with open(out, "w") as f:
    f.write("\n".join(kept) + "\n")
print(f"wrote {len(kept)} entries to {out}")
PY
