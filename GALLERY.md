# term-icon verified gallery

Every glyph below passed dual-pass verification (PTY + pixel) on macOS Terminal.app, Windows conhost/WT, and Linux xterm simultaneously.

<img src="assets/universal-catalog.png" width="440" alt="Specimen grid: every verified icon rendered on macOS, Windows, and Linux, labeled with its U+ codepoint">

Specimen grid of the full verified set: each cell shows the glyph as
actually rasterized on macOS, Windows, and Linux (left → right) with the
`U+XXXX` codepoint that activates it. Sections run drawing primitives first (Block Elements, Box Drawing, Geometric Shapes), then symbol sets, Braille, and ASCII last — each headed by its Unicode block;
ordering is block order then codepoint. Every cell's exact position, id,
and official Unicode name are mapped in
[`assets/grid-index.json`](assets/grid-index.json); the tall three-platform
strip lives at [`assets/universal-matrix.png`](assets/universal-matrix.png).

## ascii

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ! | `ascii_0021` | U+0021 | EXCLAMATION MARK | `!` |
| " | `ascii_0022` | U+0022 | QUOTATION MARK | `"` |
| # | `ascii_0023` | U+0023 | NUMBER SIGN | `#` |
| $ | `ascii_0024` | U+0024 | DOLLAR SIGN | `$` |
| % | `ascii_0025` | U+0025 | PERCENT SIGN | `%` |
| & | `ascii_0026` | U+0026 | AMPERSAND | `&` |
| ' | `ascii_0027` | U+0027 | APOSTROPHE | `'` |
| ( | `ascii_0028` | U+0028 | LEFT PARENTHESIS | `(` |
| ) | `ascii_0029` | U+0029 | RIGHT PARENTHESIS | `)` |
| * | `ascii_002A` | U+002A | ASTERISK | `*` |
| + | `ascii_002B` | U+002B | PLUS SIGN | `+` |
| , | `ascii_002C` | U+002C | COMMA | `,` |
| - | `ascii_002D` | U+002D | HYPHEN-MINUS | `-` |
| . | `ascii_002E` | U+002E | FULL STOP | `.` |
| / | `ascii_002F` | U+002F | SOLIDUS | `/` |
| 0 | `ascii_0030` | U+0030 | DIGIT ZERO | `0` |
| 1 | `ascii_0031` | U+0031 | DIGIT ONE | `1` |
| 2 | `ascii_0032` | U+0032 | DIGIT TWO | `2` |
| 3 | `ascii_0033` | U+0033 | DIGIT THREE | `3` |
| 4 | `ascii_0034` | U+0034 | DIGIT FOUR | `4` |
| 5 | `ascii_0035` | U+0035 | DIGIT FIVE | `5` |
| 6 | `ascii_0036` | U+0036 | DIGIT SIX | `6` |
| 7 | `ascii_0037` | U+0037 | DIGIT SEVEN | `7` |
| 8 | `ascii_0038` | U+0038 | DIGIT EIGHT | `8` |
| 9 | `ascii_0039` | U+0039 | DIGIT NINE | `9` |
| : | `ascii_003A` | U+003A | COLON | `:` |
| ; | `ascii_003B` | U+003B | SEMICOLON | `;` |
| < | `ascii_003C` | U+003C | LESS-THAN SIGN | `<` |
| = | `ascii_003D` | U+003D | EQUALS SIGN | `=` |
| > | `ascii_003E` | U+003E | GREATER-THAN SIGN | `>` |
| ? | `ascii_003F` | U+003F | QUESTION MARK | `?` |
| @ | `ascii_0040` | U+0040 | COMMERCIAL AT | `@` |
| A | `ascii_0041` | U+0041 | LATIN CAPITAL LETTER A | `A` |
| B | `ascii_0042` | U+0042 | LATIN CAPITAL LETTER B | `B` |
| C | `ascii_0043` | U+0043 | LATIN CAPITAL LETTER C | `C` |
| D | `ascii_0044` | U+0044 | LATIN CAPITAL LETTER D | `D` |
| E | `ascii_0045` | U+0045 | LATIN CAPITAL LETTER E | `E` |
| F | `ascii_0046` | U+0046 | LATIN CAPITAL LETTER F | `F` |
| G | `ascii_0047` | U+0047 | LATIN CAPITAL LETTER G | `G` |
| I | `ascii_0049` | U+0049 | LATIN CAPITAL LETTER I | `I` |
| J | `ascii_004A` | U+004A | LATIN CAPITAL LETTER J | `J` |
| K | `ascii_004B` | U+004B | LATIN CAPITAL LETTER K | `K` |
| L | `ascii_004C` | U+004C | LATIN CAPITAL LETTER L | `L` |
| M | `ascii_004D` | U+004D | LATIN CAPITAL LETTER M | `M` |
| N | `ascii_004E` | U+004E | LATIN CAPITAL LETTER N | `N` |
| O | `ascii_004F` | U+004F | LATIN CAPITAL LETTER O | `O` |
| P | `ascii_0050` | U+0050 | LATIN CAPITAL LETTER P | `P` |
| Q | `ascii_0051` | U+0051 | LATIN CAPITAL LETTER Q | `Q` |
| R | `ascii_0052` | U+0052 | LATIN CAPITAL LETTER R | `R` |
| S | `ascii_0053` | U+0053 | LATIN CAPITAL LETTER S | `S` |
| T | `ascii_0054` | U+0054 | LATIN CAPITAL LETTER T | `T` |
| V | `ascii_0056` | U+0056 | LATIN CAPITAL LETTER V | `V` |
| W | `ascii_0057` | U+0057 | LATIN CAPITAL LETTER W | `W` |
| X | `ascii_0058` | U+0058 | LATIN CAPITAL LETTER X | `X` |
| Y | `ascii_0059` | U+0059 | LATIN CAPITAL LETTER Y | `Y` |
| Z | `ascii_005A` | U+005A | LATIN CAPITAL LETTER Z | `Z` |
| [ | `ascii_005B` | U+005B | LEFT SQUARE BRACKET | `[` |
| \ | `ascii_005C` | U+005C | REVERSE SOLIDUS | `\` |
| ] | `ascii_005D` | U+005D | RIGHT SQUARE BRACKET | `]` |
| ^ | `ascii_005E` | U+005E | CIRCUMFLEX ACCENT | `^` |
| _ | `ascii_005F` | U+005F | LOW LINE | `_` |
| ` | `ascii_0060` | U+0060 | GRAVE ACCENT | ``` |
| a | `ascii_0061` | U+0061 | LATIN SMALL LETTER A | `a` |
| b | `ascii_0062` | U+0062 | LATIN SMALL LETTER B | `b` |
| c | `ascii_0063` | U+0063 | LATIN SMALL LETTER C | `c` |
| d | `ascii_0064` | U+0064 | LATIN SMALL LETTER D | `d` |
| e | `ascii_0065` | U+0065 | LATIN SMALL LETTER E | `e` |
| f | `ascii_0066` | U+0066 | LATIN SMALL LETTER F | `f` |
| g | `ascii_0067` | U+0067 | LATIN SMALL LETTER G | `g` |
| h | `ascii_0068` | U+0068 | LATIN SMALL LETTER H | `h` |
| i | `ascii_0069` | U+0069 | LATIN SMALL LETTER I | `i` |
| j | `ascii_006A` | U+006A | LATIN SMALL LETTER J | `j` |
| k | `ascii_006B` | U+006B | LATIN SMALL LETTER K | `k` |
| l | `ascii_006C` | U+006C | LATIN SMALL LETTER L | `l` |
| m | `ascii_006D` | U+006D | LATIN SMALL LETTER M | `m` |
| n | `ascii_006E` | U+006E | LATIN SMALL LETTER N | `n` |
| o | `ascii_006F` | U+006F | LATIN SMALL LETTER O | `o` |
| p | `ascii_0070` | U+0070 | LATIN SMALL LETTER P | `p` |
| q | `ascii_0071` | U+0071 | LATIN SMALL LETTER Q | `q` |
| r | `ascii_0072` | U+0072 | LATIN SMALL LETTER R | `r` |
| s | `ascii_0073` | U+0073 | LATIN SMALL LETTER S | `s` |
| t | `ascii_0074` | U+0074 | LATIN SMALL LETTER T | `t` |
| u | `ascii_0075` | U+0075 | LATIN SMALL LETTER U | `u` |
| v | `ascii_0076` | U+0076 | LATIN SMALL LETTER V | `v` |
| w | `ascii_0077` | U+0077 | LATIN SMALL LETTER W | `w` |
| x | `ascii_0078` | U+0078 | LATIN SMALL LETTER X | `x` |
| y | `ascii_0079` | U+0079 | LATIN SMALL LETTER Y | `y` |
| z | `ascii_007A` | U+007A | LATIN SMALL LETTER Z | `z` |
| { | `ascii_007B` | U+007B | LEFT CURLY BRACKET | `{` |
| | | `ascii_007C` | U+007C | VERTICAL LINE | `|` |
| } | `ascii_007D` | U+007D | RIGHT CURLY BRACKET | `}` |
| ~ | `ascii_007E` | U+007E | TILDE | `~` |

## arrows

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ← | `arrow_2190` | U+2190 | LEFTWARDS ARROW | `<` |
| ↑ | `arrow_2191` | U+2191 | UPWARDS ARROW | `^` |
| → | `arrow_2192` | U+2192 | RIGHTWARDS ARROW | `>` |
| ↓ | `arrow_2193` | U+2193 | DOWNWARDS ARROW | `v` |
| ↔ | `arrow_2194` | U+2194 | LEFT RIGHT ARROW | `-` |
| ↕ | `arrow_2195` | U+2195 | UP DOWN ARROW | `*` |
| ↖ | `arrow_2196` | U+2196 | NORTH WEST ARROW | `*` |
| ↗ | `arrow_2197` | U+2197 | NORTH EAST ARROW | `*` |
| ↘ | `arrow_2198` | U+2198 | SOUTH EAST ARROW | `*` |
| ↙ | `arrow_2199` | U+2199 | SOUTH WEST ARROW | `*` |
| ↚ | `arrow_219A` | U+219A | LEFTWARDS ARROW WITH STROKE | `*` |
| ↛ | `arrow_219B` | U+219B | RIGHTWARDS ARROW WITH STROKE | `*` |
| ↜ | `arrow_219C` | U+219C | LEFTWARDS WAVE ARROW | `*` |
| ↝ | `arrow_219D` | U+219D | RIGHTWARDS WAVE ARROW | `*` |
| ↞ | `arrow_219E` | U+219E | LEFTWARDS TWO HEADED ARROW | `*` |
| ↟ | `arrow_219F` | U+219F | UPWARDS TWO HEADED ARROW | `*` |
| ↠ | `arrow_21A0` | U+21A0 | RIGHTWARDS TWO HEADED ARROW | `*` |
| ↡ | `arrow_21A1` | U+21A1 | DOWNWARDS TWO HEADED ARROW | `*` |
| ↢ | `arrow_21A2` | U+21A2 | LEFTWARDS ARROW WITH TAIL | `*` |
| ↣ | `arrow_21A3` | U+21A3 | RIGHTWARDS ARROW WITH TAIL | `*` |
| ↤ | `arrow_21A4` | U+21A4 | LEFTWARDS ARROW FROM BAR | `*` |
| ↥ | `arrow_21A5` | U+21A5 | UPWARDS ARROW FROM BAR | `*` |
| ↦ | `arrow_21A6` | U+21A6 | RIGHTWARDS ARROW FROM BAR | `*` |
| ↧ | `arrow_21A7` | U+21A7 | DOWNWARDS ARROW FROM BAR | `*` |
| ↨ | `arrow_21A8` | U+21A8 | UP DOWN ARROW WITH BASE | `*` |
| ↩ | `arrow_21A9` | U+21A9 | LEFTWARDS ARROW WITH HOOK | `*` |
| ↪ | `arrow_21AA` | U+21AA | RIGHTWARDS ARROW WITH HOOK | `*` |
| ↫ | `arrow_21AB` | U+21AB | LEFTWARDS ARROW WITH LOOP | `*` |
| ↬ | `arrow_21AC` | U+21AC | RIGHTWARDS ARROW WITH LOOP | `*` |
| ↭ | `arrow_21AD` | U+21AD | LEFT RIGHT WAVE ARROW | `*` |
| ↮ | `arrow_21AE` | U+21AE | LEFT RIGHT ARROW WITH STROKE | `*` |
| ↯ | `arrow_21AF` | U+21AF | DOWNWARDS ZIGZAG ARROW | `*` |
| ↰ | `arrow_21B0` | U+21B0 | UPWARDS ARROW WITH TIP LEFTWARDS | `*` |
| ↱ | `arrow_21B1` | U+21B1 | UPWARDS ARROW WITH TIP RIGHTWARDS | `*` |
| ↲ | `arrow_21B2` | U+21B2 | DOWNWARDS ARROW WITH TIP LEFTWARDS | `*` |
| ↳ | `arrow_21B3` | U+21B3 | DOWNWARDS ARROW WITH TIP RIGHTWARDS | `*` |
| ↴ | `arrow_21B4` | U+21B4 | RIGHTWARDS ARROW WITH CORNER DOWNWARDS | `*` |
| ↵ | `arrow_21B5` | U+21B5 | DOWNWARDS ARROW WITH CORNER LEFTWARDS | `*` |
| ↶ | `arrow_21B6` | U+21B6 | ANTICLOCKWISE TOP SEMICIRCLE ARROW | `*` |
| ↷ | `arrow_21B7` | U+21B7 | CLOCKWISE TOP SEMICIRCLE ARROW | `*` |
| ↸ | `arrow_21B8` | U+21B8 | NORTH WEST ARROW TO LONG BAR | `*` |
| ↹ | `arrow_21B9` | U+21B9 | LEFTWARDS ARROW TO BAR OVER RIGHTWARDS ARROW TO BAR | `*` |
| ↺ | `arrow_21BA` | U+21BA | ANTICLOCKWISE OPEN CIRCLE ARROW | `*` |
| ↻ | `arrow_21BB` | U+21BB | CLOCKWISE OPEN CIRCLE ARROW | `*` |
| ↼ | `arrow_21BC` | U+21BC | LEFTWARDS HARPOON WITH BARB UPWARDS | `*` |
| ↽ | `arrow_21BD` | U+21BD | LEFTWARDS HARPOON WITH BARB DOWNWARDS | `*` |
| ↾ | `arrow_21BE` | U+21BE | UPWARDS HARPOON WITH BARB RIGHTWARDS | `*` |
| ↿ | `arrow_21BF` | U+21BF | UPWARDS HARPOON WITH BARB LEFTWARDS | `*` |
| ⇀ | `arrow_21C0` | U+21C0 | RIGHTWARDS HARPOON WITH BARB UPWARDS | `*` |
| ⇁ | `arrow_21C1` | U+21C1 | RIGHTWARDS HARPOON WITH BARB DOWNWARDS | `*` |
| ⇂ | `arrow_21C2` | U+21C2 | DOWNWARDS HARPOON WITH BARB RIGHTWARDS | `*` |
| ⇃ | `arrow_21C3` | U+21C3 | DOWNWARDS HARPOON WITH BARB LEFTWARDS | `*` |
| ⇄ | `arrow_21C4` | U+21C4 | RIGHTWARDS ARROW OVER LEFTWARDS ARROW | `*` |
| ⇅ | `arrow_21C5` | U+21C5 | UPWARDS ARROW LEFTWARDS OF DOWNWARDS ARROW | `*` |
| ⇆ | `arrow_21C6` | U+21C6 | LEFTWARDS ARROW OVER RIGHTWARDS ARROW | `*` |
| ⇇ | `arrow_21C7` | U+21C7 | LEFTWARDS PAIRED ARROWS | `*` |
| ⇈ | `arrow_21C8` | U+21C8 | UPWARDS PAIRED ARROWS | `*` |
| ⇉ | `arrow_21C9` | U+21C9 | RIGHTWARDS PAIRED ARROWS | `*` |
| ⇊ | `arrow_21CA` | U+21CA | DOWNWARDS PAIRED ARROWS | `*` |
| ⇋ | `arrow_21CB` | U+21CB | LEFTWARDS HARPOON OVER RIGHTWARDS HARPOON | `*` |
| ⇌ | `arrow_21CC` | U+21CC | RIGHTWARDS HARPOON OVER LEFTWARDS HARPOON | `*` |
| ⇍ | `arrow_21CD` | U+21CD | LEFTWARDS DOUBLE ARROW WITH STROKE | `*` |
| ⇎ | `arrow_21CE` | U+21CE | LEFT RIGHT DOUBLE ARROW WITH STROKE | `*` |
| ⇏ | `arrow_21CF` | U+21CF | RIGHTWARDS DOUBLE ARROW WITH STROKE | `*` |
| ⇐ | `arrow_21D0` | U+21D0 | LEFTWARDS DOUBLE ARROW | `*` |
| ⇑ | `arrow_21D1` | U+21D1 | UPWARDS DOUBLE ARROW | `*` |
| ⇒ | `arrow_21D2` | U+21D2 | RIGHTWARDS DOUBLE ARROW | `*` |
| ⇓ | `arrow_21D3` | U+21D3 | DOWNWARDS DOUBLE ARROW | `*` |
| ⇔ | `arrow_21D4` | U+21D4 | LEFT RIGHT DOUBLE ARROW | `*` |
| ⇕ | `arrow_21D5` | U+21D5 | UP DOWN DOUBLE ARROW | `*` |
| ⇖ | `arrow_21D6` | U+21D6 | NORTH WEST DOUBLE ARROW | `*` |
| ⇗ | `arrow_21D7` | U+21D7 | NORTH EAST DOUBLE ARROW | `*` |
| ⇘ | `arrow_21D8` | U+21D8 | SOUTH EAST DOUBLE ARROW | `*` |
| ⇙ | `arrow_21D9` | U+21D9 | SOUTH WEST DOUBLE ARROW | `*` |
| ⇚ | `arrow_21DA` | U+21DA | LEFTWARDS TRIPLE ARROW | `*` |
| ⇛ | `arrow_21DB` | U+21DB | RIGHTWARDS TRIPLE ARROW | `*` |
| ⇜ | `arrow_21DC` | U+21DC | LEFTWARDS SQUIGGLE ARROW | `*` |
| ⇝ | `arrow_21DD` | U+21DD | RIGHTWARDS SQUIGGLE ARROW | `*` |
| ⇞ | `arrow_21DE` | U+21DE | UPWARDS ARROW WITH DOUBLE STROKE | `*` |
| ⇟ | `arrow_21DF` | U+21DF | DOWNWARDS ARROW WITH DOUBLE STROKE | `*` |
| ⇠ | `arrow_21E0` | U+21E0 | LEFTWARDS DASHED ARROW | `*` |
| ⇡ | `arrow_21E1` | U+21E1 | UPWARDS DASHED ARROW | `*` |
| ⇢ | `arrow_21E2` | U+21E2 | RIGHTWARDS DASHED ARROW | `*` |
| ⇣ | `arrow_21E3` | U+21E3 | DOWNWARDS DASHED ARROW | `*` |
| ⇤ | `arrow_21E4` | U+21E4 | LEFTWARDS ARROW TO BAR | `*` |
| ⇥ | `arrow_21E5` | U+21E5 | RIGHTWARDS ARROW TO BAR | `*` |
| ⇦ | `arrow_21E6` | U+21E6 | LEFTWARDS WHITE ARROW | `*` |
| ⇧ | `arrow_21E7` | U+21E7 | UPWARDS WHITE ARROW | `*` |
| ⇨ | `arrow_21E8` | U+21E8 | RIGHTWARDS WHITE ARROW | `*` |
| ⇩ | `arrow_21E9` | U+21E9 | DOWNWARDS WHITE ARROW | `*` |
| ⇪ | `arrow_21EA` | U+21EA | UPWARDS WHITE ARROW FROM BAR | `*` |
| ⇫ | `arrow_21EB` | U+21EB | UPWARDS WHITE ARROW ON PEDESTAL | `*` |
| ⇬ | `arrow_21EC` | U+21EC | UPWARDS WHITE ARROW ON PEDESTAL WITH HORIZONTAL BAR | `*` |
| ⇭ | `arrow_21ED` | U+21ED | UPWARDS WHITE ARROW ON PEDESTAL WITH VERTICAL BAR | `*` |
| ⇮ | `arrow_21EE` | U+21EE | UPWARDS WHITE DOUBLE ARROW | `*` |
| ⇯ | `arrow_21EF` | U+21EF | UPWARDS WHITE DOUBLE ARROW ON PEDESTAL | `*` |
| ⇰ | `arrow_21F0` | U+21F0 | RIGHTWARDS WHITE ARROW FROM WALL | `*` |
| ⇱ | `arrow_21F1` | U+21F1 | NORTH WEST ARROW TO CORNER | `*` |
| ⇲ | `arrow_21F2` | U+21F2 | SOUTH EAST ARROW TO CORNER | `*` |
| ⇳ | `arrow_21F3` | U+21F3 | UP DOWN WHITE ARROW | `*` |
| ⇴ | `arrow_21F4` | U+21F4 | RIGHT ARROW WITH SMALL CIRCLE | `*` |
| ⇵ | `arrow_21F5` | U+21F5 | DOWNWARDS ARROW LEFTWARDS OF UPWARDS ARROW | `*` |
| ⇶ | `arrow_21F6` | U+21F6 | THREE RIGHTWARDS ARROWS | `*` |
| ⇷ | `arrow_21F7` | U+21F7 | LEFTWARDS ARROW WITH VERTICAL STROKE | `*` |
| ⇸ | `arrow_21F8` | U+21F8 | RIGHTWARDS ARROW WITH VERTICAL STROKE | `*` |
| ⇹ | `arrow_21F9` | U+21F9 | LEFT RIGHT ARROW WITH VERTICAL STROKE | `*` |
| ⇺ | `arrow_21FA` | U+21FA | LEFTWARDS ARROW WITH DOUBLE VERTICAL STROKE | `*` |
| ⇻ | `arrow_21FB` | U+21FB | RIGHTWARDS ARROW WITH DOUBLE VERTICAL STROKE | `*` |
| ⇼ | `arrow_21FC` | U+21FC | LEFT RIGHT ARROW WITH DOUBLE VERTICAL STROKE | `*` |
| ⇽ | `arrow_21FD` | U+21FD | LEFTWARDS OPEN-HEADED ARROW | `*` |
| ⇾ | `arrow_21FE` | U+21FE | RIGHTWARDS OPEN-HEADED ARROW | `*` |
| ⇿ | `arrow_21FF` | U+21FF | LEFT RIGHT OPEN-HEADED ARROW | `*` |

## box_drawing

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ─ | `box_2500` | U+2500 | BOX DRAWINGS LIGHT HORIZONTAL | `-` |
| ━ | `box_2501` | U+2501 | BOX DRAWINGS HEAVY HORIZONTAL | `+` |
| │ | `box_2502` | U+2502 | BOX DRAWINGS LIGHT VERTICAL | `|` |
| ┃ | `box_2503` | U+2503 | BOX DRAWINGS HEAVY VERTICAL | `+` |
| ┄ | `box_2504` | U+2504 | BOX DRAWINGS LIGHT TRIPLE DASH HORIZONTAL | `+` |
| ┅ | `box_2505` | U+2505 | BOX DRAWINGS HEAVY TRIPLE DASH HORIZONTAL | `+` |
| ┆ | `box_2506` | U+2506 | BOX DRAWINGS LIGHT TRIPLE DASH VERTICAL | `+` |
| ┇ | `box_2507` | U+2507 | BOX DRAWINGS HEAVY TRIPLE DASH VERTICAL | `+` |
| ┈ | `box_2508` | U+2508 | BOX DRAWINGS LIGHT QUADRUPLE DASH HORIZONTAL | `+` |
| ┉ | `box_2509` | U+2509 | BOX DRAWINGS HEAVY QUADRUPLE DASH HORIZONTAL | `+` |
| ┊ | `box_250A` | U+250A | BOX DRAWINGS LIGHT QUADRUPLE DASH VERTICAL | `+` |
| ┋ | `box_250B` | U+250B | BOX DRAWINGS HEAVY QUADRUPLE DASH VERTICAL | `+` |
| ┌ | `box_250C` | U+250C | BOX DRAWINGS LIGHT DOWN AND RIGHT | `+` |
| ┍ | `box_250D` | U+250D | BOX DRAWINGS DOWN LIGHT AND RIGHT HEAVY | `+` |
| ┎ | `box_250E` | U+250E | BOX DRAWINGS DOWN HEAVY AND RIGHT LIGHT | `+` |
| ┏ | `box_250F` | U+250F | BOX DRAWINGS HEAVY DOWN AND RIGHT | `+` |
| ┐ | `box_2510` | U+2510 | BOX DRAWINGS LIGHT DOWN AND LEFT | `+` |
| ┑ | `box_2511` | U+2511 | BOX DRAWINGS DOWN LIGHT AND LEFT HEAVY | `+` |
| ┒ | `box_2512` | U+2512 | BOX DRAWINGS DOWN HEAVY AND LEFT LIGHT | `+` |
| ┓ | `box_2513` | U+2513 | BOX DRAWINGS HEAVY DOWN AND LEFT | `+` |
| └ | `box_2514` | U+2514 | BOX DRAWINGS LIGHT UP AND RIGHT | `+` |
| ┕ | `box_2515` | U+2515 | BOX DRAWINGS UP LIGHT AND RIGHT HEAVY | `+` |
| ┖ | `box_2516` | U+2516 | BOX DRAWINGS UP HEAVY AND RIGHT LIGHT | `+` |
| ┗ | `box_2517` | U+2517 | BOX DRAWINGS HEAVY UP AND RIGHT | `+` |
| ┘ | `box_2518` | U+2518 | BOX DRAWINGS LIGHT UP AND LEFT | `+` |
| ┙ | `box_2519` | U+2519 | BOX DRAWINGS UP LIGHT AND LEFT HEAVY | `+` |
| ┚ | `box_251A` | U+251A | BOX DRAWINGS UP HEAVY AND LEFT LIGHT | `+` |
| ┛ | `box_251B` | U+251B | BOX DRAWINGS HEAVY UP AND LEFT | `+` |
| ├ | `box_251C` | U+251C | BOX DRAWINGS LIGHT VERTICAL AND RIGHT | `+` |
| ┝ | `box_251D` | U+251D | BOX DRAWINGS VERTICAL LIGHT AND RIGHT HEAVY | `+` |
| ┞ | `box_251E` | U+251E | BOX DRAWINGS UP HEAVY AND RIGHT DOWN LIGHT | `+` |
| ┟ | `box_251F` | U+251F | BOX DRAWINGS DOWN HEAVY AND RIGHT UP LIGHT | `+` |
| ┠ | `box_2520` | U+2520 | BOX DRAWINGS VERTICAL HEAVY AND RIGHT LIGHT | `+` |
| ┡ | `box_2521` | U+2521 | BOX DRAWINGS DOWN LIGHT AND RIGHT UP HEAVY | `+` |
| ┢ | `box_2522` | U+2522 | BOX DRAWINGS UP LIGHT AND RIGHT DOWN HEAVY | `+` |
| ┣ | `box_2523` | U+2523 | BOX DRAWINGS HEAVY VERTICAL AND RIGHT | `+` |
| ┤ | `box_2524` | U+2524 | BOX DRAWINGS LIGHT VERTICAL AND LEFT | `+` |
| ┥ | `box_2525` | U+2525 | BOX DRAWINGS VERTICAL LIGHT AND LEFT HEAVY | `+` |
| ┦ | `box_2526` | U+2526 | BOX DRAWINGS UP HEAVY AND LEFT DOWN LIGHT | `+` |
| ┧ | `box_2527` | U+2527 | BOX DRAWINGS DOWN HEAVY AND LEFT UP LIGHT | `+` |
| ┨ | `box_2528` | U+2528 | BOX DRAWINGS VERTICAL HEAVY AND LEFT LIGHT | `+` |
| ┩ | `box_2529` | U+2529 | BOX DRAWINGS DOWN LIGHT AND LEFT UP HEAVY | `+` |
| ┪ | `box_252A` | U+252A | BOX DRAWINGS UP LIGHT AND LEFT DOWN HEAVY | `+` |
| ┫ | `box_252B` | U+252B | BOX DRAWINGS HEAVY VERTICAL AND LEFT | `+` |
| ┬ | `box_252C` | U+252C | BOX DRAWINGS LIGHT DOWN AND HORIZONTAL | `+` |
| ┭ | `box_252D` | U+252D | BOX DRAWINGS LEFT HEAVY AND RIGHT DOWN LIGHT | `+` |
| ┮ | `box_252E` | U+252E | BOX DRAWINGS RIGHT HEAVY AND LEFT DOWN LIGHT | `+` |
| ┯ | `box_252F` | U+252F | BOX DRAWINGS DOWN LIGHT AND HORIZONTAL HEAVY | `+` |
| ┰ | `box_2530` | U+2530 | BOX DRAWINGS DOWN HEAVY AND HORIZONTAL LIGHT | `+` |
| ┱ | `box_2531` | U+2531 | BOX DRAWINGS RIGHT LIGHT AND LEFT DOWN HEAVY | `+` |
| ┲ | `box_2532` | U+2532 | BOX DRAWINGS LEFT LIGHT AND RIGHT DOWN HEAVY | `+` |
| ┳ | `box_2533` | U+2533 | BOX DRAWINGS HEAVY DOWN AND HORIZONTAL | `+` |
| ┴ | `box_2534` | U+2534 | BOX DRAWINGS LIGHT UP AND HORIZONTAL | `+` |
| ┵ | `box_2535` | U+2535 | BOX DRAWINGS LEFT HEAVY AND RIGHT UP LIGHT | `+` |
| ┶ | `box_2536` | U+2536 | BOX DRAWINGS RIGHT HEAVY AND LEFT UP LIGHT | `+` |
| ┷ | `box_2537` | U+2537 | BOX DRAWINGS UP LIGHT AND HORIZONTAL HEAVY | `+` |
| ┸ | `box_2538` | U+2538 | BOX DRAWINGS UP HEAVY AND HORIZONTAL LIGHT | `+` |
| ┹ | `box_2539` | U+2539 | BOX DRAWINGS RIGHT LIGHT AND LEFT UP HEAVY | `+` |
| ┺ | `box_253A` | U+253A | BOX DRAWINGS LEFT LIGHT AND RIGHT UP HEAVY | `+` |
| ┻ | `box_253B` | U+253B | BOX DRAWINGS HEAVY UP AND HORIZONTAL | `+` |
| ┼ | `box_253C` | U+253C | BOX DRAWINGS LIGHT VERTICAL AND HORIZONTAL | `+` |
| ┽ | `box_253D` | U+253D | BOX DRAWINGS LEFT HEAVY AND RIGHT VERTICAL LIGHT | `+` |
| ┾ | `box_253E` | U+253E | BOX DRAWINGS RIGHT HEAVY AND LEFT VERTICAL LIGHT | `+` |
| ┿ | `box_253F` | U+253F | BOX DRAWINGS VERTICAL LIGHT AND HORIZONTAL HEAVY | `+` |
| ╀ | `box_2540` | U+2540 | BOX DRAWINGS UP HEAVY AND DOWN HORIZONTAL LIGHT | `+` |
| ╁ | `box_2541` | U+2541 | BOX DRAWINGS DOWN HEAVY AND UP HORIZONTAL LIGHT | `+` |
| ╂ | `box_2542` | U+2542 | BOX DRAWINGS VERTICAL HEAVY AND HORIZONTAL LIGHT | `+` |
| ╃ | `box_2543` | U+2543 | BOX DRAWINGS LEFT UP HEAVY AND RIGHT DOWN LIGHT | `+` |
| ╄ | `box_2544` | U+2544 | BOX DRAWINGS RIGHT UP HEAVY AND LEFT DOWN LIGHT | `+` |
| ╅ | `box_2545` | U+2545 | BOX DRAWINGS LEFT DOWN HEAVY AND RIGHT UP LIGHT | `+` |
| ╆ | `box_2546` | U+2546 | BOX DRAWINGS RIGHT DOWN HEAVY AND LEFT UP LIGHT | `+` |
| ╇ | `box_2547` | U+2547 | BOX DRAWINGS DOWN LIGHT AND UP HORIZONTAL HEAVY | `+` |
| ╈ | `box_2548` | U+2548 | BOX DRAWINGS UP LIGHT AND DOWN HORIZONTAL HEAVY | `+` |
| ╉ | `box_2549` | U+2549 | BOX DRAWINGS RIGHT LIGHT AND LEFT VERTICAL HEAVY | `+` |
| ╊ | `box_254A` | U+254A | BOX DRAWINGS LEFT LIGHT AND RIGHT VERTICAL HEAVY | `+` |
| ╋ | `box_254B` | U+254B | BOX DRAWINGS HEAVY VERTICAL AND HORIZONTAL | `+` |
| ╌ | `box_254C` | U+254C | BOX DRAWINGS LIGHT DOUBLE DASH HORIZONTAL | `+` |
| ╍ | `box_254D` | U+254D | BOX DRAWINGS HEAVY DOUBLE DASH HORIZONTAL | `+` |
| ╎ | `box_254E` | U+254E | BOX DRAWINGS LIGHT DOUBLE DASH VERTICAL | `+` |
| ╏ | `box_254F` | U+254F | BOX DRAWINGS HEAVY DOUBLE DASH VERTICAL | `+` |
| ═ | `box_2550` | U+2550 | BOX DRAWINGS DOUBLE HORIZONTAL | `=` |
| ║ | `box_2551` | U+2551 | BOX DRAWINGS DOUBLE VERTICAL | `|` |
| ╒ | `box_2552` | U+2552 | BOX DRAWINGS DOWN SINGLE AND RIGHT DOUBLE | `+` |
| ╓ | `box_2553` | U+2553 | BOX DRAWINGS DOWN DOUBLE AND RIGHT SINGLE | `+` |
| ╔ | `box_2554` | U+2554 | BOX DRAWINGS DOUBLE DOWN AND RIGHT | `+` |
| ╕ | `box_2555` | U+2555 | BOX DRAWINGS DOWN SINGLE AND LEFT DOUBLE | `+` |
| ╖ | `box_2556` | U+2556 | BOX DRAWINGS DOWN DOUBLE AND LEFT SINGLE | `+` |
| ╗ | `box_2557` | U+2557 | BOX DRAWINGS DOUBLE DOWN AND LEFT | `+` |
| ╘ | `box_2558` | U+2558 | BOX DRAWINGS UP SINGLE AND RIGHT DOUBLE | `+` |
| ╙ | `box_2559` | U+2559 | BOX DRAWINGS UP DOUBLE AND RIGHT SINGLE | `+` |
| ╚ | `box_255A` | U+255A | BOX DRAWINGS DOUBLE UP AND RIGHT | `+` |
| ╛ | `box_255B` | U+255B | BOX DRAWINGS UP SINGLE AND LEFT DOUBLE | `+` |
| ╜ | `box_255C` | U+255C | BOX DRAWINGS UP DOUBLE AND LEFT SINGLE | `+` |
| ╝ | `box_255D` | U+255D | BOX DRAWINGS DOUBLE UP AND LEFT | `+` |
| ╞ | `box_255E` | U+255E | BOX DRAWINGS VERTICAL SINGLE AND RIGHT DOUBLE | `+` |
| ╟ | `box_255F` | U+255F | BOX DRAWINGS VERTICAL DOUBLE AND RIGHT SINGLE | `+` |
| ╠ | `box_2560` | U+2560 | BOX DRAWINGS DOUBLE VERTICAL AND RIGHT | `+` |
| ╡ | `box_2561` | U+2561 | BOX DRAWINGS VERTICAL SINGLE AND LEFT DOUBLE | `+` |
| ╢ | `box_2562` | U+2562 | BOX DRAWINGS VERTICAL DOUBLE AND LEFT SINGLE | `+` |
| ╣ | `box_2563` | U+2563 | BOX DRAWINGS DOUBLE VERTICAL AND LEFT | `+` |
| ╤ | `box_2564` | U+2564 | BOX DRAWINGS DOWN SINGLE AND HORIZONTAL DOUBLE | `+` |
| ╥ | `box_2565` | U+2565 | BOX DRAWINGS DOWN DOUBLE AND HORIZONTAL SINGLE | `+` |
| ╦ | `box_2566` | U+2566 | BOX DRAWINGS DOUBLE DOWN AND HORIZONTAL | `+` |
| ╧ | `box_2567` | U+2567 | BOX DRAWINGS UP SINGLE AND HORIZONTAL DOUBLE | `+` |
| ╨ | `box_2568` | U+2568 | BOX DRAWINGS UP DOUBLE AND HORIZONTAL SINGLE | `+` |
| ╩ | `box_2569` | U+2569 | BOX DRAWINGS DOUBLE UP AND HORIZONTAL | `+` |
| ╪ | `box_256A` | U+256A | BOX DRAWINGS VERTICAL SINGLE AND HORIZONTAL DOUBLE | `+` |
| ╫ | `box_256B` | U+256B | BOX DRAWINGS VERTICAL DOUBLE AND HORIZONTAL SINGLE | `+` |
| ╬ | `box_256C` | U+256C | BOX DRAWINGS DOUBLE VERTICAL AND HORIZONTAL | `+` |
| ╭ | `box_256D` | U+256D | BOX DRAWINGS LIGHT ARC DOWN AND RIGHT | `+` |
| ╮ | `box_256E` | U+256E | BOX DRAWINGS LIGHT ARC DOWN AND LEFT | `+` |
| ╯ | `box_256F` | U+256F | BOX DRAWINGS LIGHT ARC UP AND LEFT | `+` |
| ╰ | `box_2570` | U+2570 | BOX DRAWINGS LIGHT ARC UP AND RIGHT | `+` |
| ╱ | `box_2571` | U+2571 | BOX DRAWINGS LIGHT DIAGONAL UPPER RIGHT TO LOWER LEFT | `/` |
| ╲ | `box_2572` | U+2572 | BOX DRAWINGS LIGHT DIAGONAL UPPER LEFT TO LOWER RIGHT | `\` |
| ╳ | `box_2573` | U+2573 | BOX DRAWINGS LIGHT DIAGONAL CROSS | `x` |
| ╴ | `box_2574` | U+2574 | BOX DRAWINGS LIGHT LEFT | `+` |
| ╵ | `box_2575` | U+2575 | BOX DRAWINGS LIGHT UP | `+` |
| ╶ | `box_2576` | U+2576 | BOX DRAWINGS LIGHT RIGHT | `+` |
| ╷ | `box_2577` | U+2577 | BOX DRAWINGS LIGHT DOWN | `+` |
| ╸ | `box_2578` | U+2578 | BOX DRAWINGS HEAVY LEFT | `+` |
| ╹ | `box_2579` | U+2579 | BOX DRAWINGS HEAVY UP | `+` |
| ╺ | `box_257A` | U+257A | BOX DRAWINGS HEAVY RIGHT | `+` |
| ╻ | `box_257B` | U+257B | BOX DRAWINGS HEAVY DOWN | `+` |
| ╼ | `box_257C` | U+257C | BOX DRAWINGS LIGHT LEFT AND HEAVY RIGHT | `+` |
| ╽ | `box_257D` | U+257D | BOX DRAWINGS LIGHT UP AND HEAVY DOWN | `+` |
| ╾ | `box_257E` | U+257E | BOX DRAWINGS HEAVY LEFT AND LIGHT RIGHT | `+` |
| ╿ | `box_257F` | U+257F | BOX DRAWINGS HEAVY UP AND LIGHT DOWN | `+` |

## block_elements

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ▀ | `block_2580` | U+2580 | UPPER HALF BLOCK | `^` |
| ▁ | `block_2581` | U+2581 | LOWER ONE EIGHTH BLOCK | `#` |
| ▂ | `block_2582` | U+2582 | LOWER ONE QUARTER BLOCK | `#` |
| ▃ | `block_2583` | U+2583 | LOWER THREE EIGHTHS BLOCK | `#` |
| ▄ | `block_2584` | U+2584 | LOWER HALF BLOCK | `_` |
| ▅ | `block_2585` | U+2585 | LOWER FIVE EIGHTHS BLOCK | `#` |
| ▆ | `block_2586` | U+2586 | LOWER THREE QUARTERS BLOCK | `#` |
| ▇ | `block_2587` | U+2587 | LOWER SEVEN EIGHTHS BLOCK | `#` |
| █ | `block_2588` | U+2588 | FULL BLOCK | `#` |
| ▉ | `block_2589` | U+2589 | LEFT SEVEN EIGHTHS BLOCK | `#` |
| ▊ | `block_258A` | U+258A | LEFT THREE QUARTERS BLOCK | `#` |
| ▋ | `block_258B` | U+258B | LEFT FIVE EIGHTHS BLOCK | `#` |
| ▌ | `block_258C` | U+258C | LEFT HALF BLOCK | `[` |
| ▍ | `block_258D` | U+258D | LEFT THREE EIGHTHS BLOCK | `#` |
| ▎ | `block_258E` | U+258E | LEFT ONE QUARTER BLOCK | `#` |
| ▐ | `block_2590` | U+2590 | RIGHT HALF BLOCK | `]` |
| ░ | `block_2591` | U+2591 | LIGHT SHADE | `.` |
| ▒ | `block_2592` | U+2592 | MEDIUM SHADE | `:` |
| ▓ | `block_2593` | U+2593 | DARK SHADE | `#` |
| ▖ | `block_2596` | U+2596 | QUADRANT LOWER LEFT | `#` |
| ▗ | `block_2597` | U+2597 | QUADRANT LOWER RIGHT | `#` |
| ▘ | `block_2598` | U+2598 | QUADRANT UPPER LEFT | `#` |
| ▙ | `block_2599` | U+2599 | QUADRANT UPPER LEFT AND LOWER LEFT AND LOWER RIGHT | `#` |
| ▚ | `block_259A` | U+259A | QUADRANT UPPER LEFT AND LOWER RIGHT | `#` |
| ▛ | `block_259B` | U+259B | QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER LEFT | `#` |
| ▜ | `block_259C` | U+259C | QUADRANT UPPER LEFT AND UPPER RIGHT AND LOWER RIGHT | `#` |
| ▝ | `block_259D` | U+259D | QUADRANT UPPER RIGHT | `#` |
| ▞ | `block_259E` | U+259E | QUADRANT UPPER RIGHT AND LOWER LEFT | `#` |
| ▟ | `block_259F` | U+259F | QUADRANT UPPER RIGHT AND LOWER LEFT AND LOWER RIGHT | `#` |

## geometric_shapes

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ■ | `geometric_25A0` | U+25A0 | BLACK SQUARE | `#` |
| ▢ | `geometric_25A2` | U+25A2 | WHITE SQUARE WITH ROUNDED CORNERS | `#` |
| ▣ | `geometric_25A3` | U+25A3 | WHITE SQUARE CONTAINING BLACK SMALL SQUARE | `#` |
| ▤ | `geometric_25A4` | U+25A4 | SQUARE WITH HORIZONTAL FILL | `#` |
| ▥ | `geometric_25A5` | U+25A5 | SQUARE WITH VERTICAL FILL | `#` |
| ▦ | `geometric_25A6` | U+25A6 | SQUARE WITH ORTHOGONAL CROSSHATCH FILL | `#` |
| ▧ | `geometric_25A7` | U+25A7 | SQUARE WITH UPPER LEFT TO LOWER RIGHT FILL | `#` |
| ▨ | `geometric_25A8` | U+25A8 | SQUARE WITH UPPER RIGHT TO LOWER LEFT FILL | `#` |
| ▩ | `geometric_25A9` | U+25A9 | SQUARE WITH DIAGONAL CROSSHATCH FILL | `#` |
| ▪ | `geometric_25AA` | U+25AA | BLACK SMALL SQUARE | `#` |
| ▫ | `geometric_25AB` | U+25AB | WHITE SMALL SQUARE | `#` |
| ▬ | `geometric_25AC` | U+25AC | BLACK RECTANGLE | `#` |
| ▭ | `geometric_25AD` | U+25AD | WHITE RECTANGLE | `#` |
| ▮ | `geometric_25AE` | U+25AE | BLACK VERTICAL RECTANGLE | `#` |
| ▰ | `geometric_25B0` | U+25B0 | BLACK PARALLELOGRAM | `#` |
| ▱ | `geometric_25B1` | U+25B1 | WHITE PARALLELOGRAM | `#` |
| ▲ | `geometric_25B2` | U+25B2 | BLACK UP-POINTING TRIANGLE | `^` |
| △ | `geometric_25B3` | U+25B3 | WHITE UP-POINTING TRIANGLE | `#` |
| ▴ | `geometric_25B4` | U+25B4 | BLACK UP-POINTING SMALL TRIANGLE | `#` |
| ▵ | `geometric_25B5` | U+25B5 | WHITE UP-POINTING SMALL TRIANGLE | `#` |
| ▶ | `geometric_25B6` | U+25B6 | BLACK RIGHT-POINTING TRIANGLE | `>` |
| ▷ | `geometric_25B7` | U+25B7 | WHITE RIGHT-POINTING TRIANGLE | `#` |
| ▸ | `geometric_25B8` | U+25B8 | BLACK RIGHT-POINTING SMALL TRIANGLE | `#` |
| ▹ | `geometric_25B9` | U+25B9 | WHITE RIGHT-POINTING SMALL TRIANGLE | `#` |
| ► | `geometric_25BA` | U+25BA | BLACK RIGHT-POINTING POINTER | `#` |
| ▻ | `geometric_25BB` | U+25BB | WHITE RIGHT-POINTING POINTER | `#` |
| ▼ | `geometric_25BC` | U+25BC | BLACK DOWN-POINTING TRIANGLE | `v` |
| ▽ | `geometric_25BD` | U+25BD | WHITE DOWN-POINTING TRIANGLE | `#` |
| ▾ | `geometric_25BE` | U+25BE | BLACK DOWN-POINTING SMALL TRIANGLE | `#` |
| ▿ | `geometric_25BF` | U+25BF | WHITE DOWN-POINTING SMALL TRIANGLE | `#` |
| ◀ | `geometric_25C0` | U+25C0 | BLACK LEFT-POINTING TRIANGLE | `<` |
| ◁ | `geometric_25C1` | U+25C1 | WHITE LEFT-POINTING TRIANGLE | `#` |
| ◂ | `geometric_25C2` | U+25C2 | BLACK LEFT-POINTING SMALL TRIANGLE | `#` |
| ◃ | `geometric_25C3` | U+25C3 | WHITE LEFT-POINTING SMALL TRIANGLE | `#` |
| ◄ | `geometric_25C4` | U+25C4 | BLACK LEFT-POINTING POINTER | `#` |
| ◅ | `geometric_25C5` | U+25C5 | WHITE LEFT-POINTING POINTER | `#` |
| ◆ | `geometric_25C6` | U+25C6 | BLACK DIAMOND | `*` |
| ◇ | `geometric_25C7` | U+25C7 | WHITE DIAMOND | `#` |
| ◈ | `geometric_25C8` | U+25C8 | WHITE DIAMOND CONTAINING BLACK SMALL DIAMOND | `#` |
| ◉ | `geometric_25C9` | U+25C9 | FISHEYE | `#` |
| ◊ | `geometric_25CA` | U+25CA | LOZENGE | `#` |
| ○ | `geometric_25CB` | U+25CB | WHITE CIRCLE | `o` |
| ◌ | `geometric_25CC` | U+25CC | DOTTED CIRCLE | `#` |
| ◍ | `geometric_25CD` | U+25CD | CIRCLE WITH VERTICAL FILL | `#` |
| ◎ | `geometric_25CE` | U+25CE | BULLSEYE | `#` |
| ● | `geometric_25CF` | U+25CF | BLACK CIRCLE | `o` |
| ◐ | `geometric_25D0` | U+25D0 | CIRCLE WITH LEFT HALF BLACK | `#` |
| ◑ | `geometric_25D1` | U+25D1 | CIRCLE WITH RIGHT HALF BLACK | `#` |
| ◒ | `geometric_25D2` | U+25D2 | CIRCLE WITH LOWER HALF BLACK | `#` |
| ◓ | `geometric_25D3` | U+25D3 | CIRCLE WITH UPPER HALF BLACK | `#` |
| ◔ | `geometric_25D4` | U+25D4 | CIRCLE WITH UPPER RIGHT QUADRANT BLACK | `#` |
| ◕ | `geometric_25D5` | U+25D5 | CIRCLE WITH ALL BUT UPPER LEFT QUADRANT BLACK | `#` |
| ◖ | `geometric_25D6` | U+25D6 | LEFT HALF BLACK CIRCLE | `#` |
| ◗ | `geometric_25D7` | U+25D7 | RIGHT HALF BLACK CIRCLE | `#` |
| ◘ | `geometric_25D8` | U+25D8 | INVERSE BULLET | `#` |
| ◙ | `geometric_25D9` | U+25D9 | INVERSE WHITE CIRCLE | `#` |
| ◚ | `geometric_25DA` | U+25DA | UPPER HALF INVERSE WHITE CIRCLE | `#` |
| ◛ | `geometric_25DB` | U+25DB | LOWER HALF INVERSE WHITE CIRCLE | `#` |
| ◜ | `geometric_25DC` | U+25DC | UPPER LEFT QUADRANT CIRCULAR ARC | `#` |
| ◝ | `geometric_25DD` | U+25DD | UPPER RIGHT QUADRANT CIRCULAR ARC | `#` |
| ◞ | `geometric_25DE` | U+25DE | LOWER RIGHT QUADRANT CIRCULAR ARC | `#` |
| ◟ | `geometric_25DF` | U+25DF | LOWER LEFT QUADRANT CIRCULAR ARC | `#` |
| ◠ | `geometric_25E0` | U+25E0 | UPPER HALF CIRCLE | `#` |
| ◡ | `geometric_25E1` | U+25E1 | LOWER HALF CIRCLE | `#` |
| ◢ | `geometric_25E2` | U+25E2 | BLACK LOWER RIGHT TRIANGLE | `#` |
| ◣ | `geometric_25E3` | U+25E3 | BLACK LOWER LEFT TRIANGLE | `#` |
| ◤ | `geometric_25E4` | U+25E4 | BLACK UPPER LEFT TRIANGLE | `#` |
| ◥ | `geometric_25E5` | U+25E5 | BLACK UPPER RIGHT TRIANGLE | `#` |
| ◦ | `geometric_25E6` | U+25E6 | WHITE BULLET | `#` |
| ◧ | `geometric_25E7` | U+25E7 | SQUARE WITH LEFT HALF BLACK | `#` |
| ◨ | `geometric_25E8` | U+25E8 | SQUARE WITH RIGHT HALF BLACK | `#` |
| ◩ | `geometric_25E9` | U+25E9 | SQUARE WITH UPPER LEFT DIAGONAL HALF BLACK | `#` |
| ◪ | `geometric_25EA` | U+25EA | SQUARE WITH LOWER RIGHT DIAGONAL HALF BLACK | `#` |
| ◫ | `geometric_25EB` | U+25EB | WHITE SQUARE WITH VERTICAL BISECTING LINE | `#` |
| ◬ | `geometric_25EC` | U+25EC | WHITE UP-POINTING TRIANGLE WITH DOT | `#` |
| ◭ | `geometric_25ED` | U+25ED | UP-POINTING TRIANGLE WITH LEFT HALF BLACK | `#` |
| ◮ | `geometric_25EE` | U+25EE | UP-POINTING TRIANGLE WITH RIGHT HALF BLACK | `#` |
| ◯ | `geometric_25EF` | U+25EF | LARGE CIRCLE | `#` |
| ◳ | `geometric_25F3` | U+25F3 | WHITE SQUARE WITH UPPER RIGHT QUADRANT | `#` |
| ◴ | `geometric_25F4` | U+25F4 | WHITE CIRCLE WITH UPPER LEFT QUADRANT | `#` |
| ◵ | `geometric_25F5` | U+25F5 | WHITE CIRCLE WITH LOWER LEFT QUADRANT | `#` |
| ◶ | `geometric_25F6` | U+25F6 | WHITE CIRCLE WITH LOWER RIGHT QUADRANT | `#` |
| ◷ | `geometric_25F7` | U+25F7 | WHITE CIRCLE WITH UPPER RIGHT QUADRANT | `#` |
| ◸ | `geometric_25F8` | U+25F8 | UPPER LEFT TRIANGLE | `#` |
| ◹ | `geometric_25F9` | U+25F9 | UPPER RIGHT TRIANGLE | `#` |
| ◺ | `geometric_25FA` | U+25FA | LOWER LEFT TRIANGLE | `#` |
| ◻ | `geometric_25FB` | U+25FB | WHITE MEDIUM SQUARE | `#` |
| ◼ | `geometric_25FC` | U+25FC | BLACK MEDIUM SQUARE | `#` |
| ◿ | `geometric_25FF` | U+25FF | LOWER RIGHT TRIANGLE | `#` |

## misc_symbols

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ☀ | `misc_2600` | U+2600 | BLACK SUN WITH RAYS | `*` |
| ☁ | `misc_2601` | U+2601 | CLOUD | `*` |
| ☂ | `misc_2602` | U+2602 | UMBRELLA | `*` |
| ☃ | `misc_2603` | U+2603 | SNOWMAN | `*` |
| ☄ | `misc_2604` | U+2604 | COMET | `*` |
| ★ | `misc_2605` | U+2605 | BLACK STAR | `*` |
| ☆ | `misc_2606` | U+2606 | WHITE STAR | `*` |
| ☇ | `misc_2607` | U+2607 | LIGHTNING | `*` |
| ☈ | `misc_2608` | U+2608 | THUNDERSTORM | `*` |
| ☉ | `misc_2609` | U+2609 | SUN | `*` |
| ☊ | `misc_260A` | U+260A | ASCENDING NODE | `*` |
| ☋ | `misc_260B` | U+260B | DESCENDING NODE | `*` |
| ☌ | `misc_260C` | U+260C | CONJUNCTION | `*` |
| ☍ | `misc_260D` | U+260D | OPPOSITION | `*` |
| ☎ | `misc_260E` | U+260E | BLACK TELEPHONE | `*` |
| ☏ | `misc_260F` | U+260F | WHITE TELEPHONE | `*` |
| ☓ | `misc_2613` | U+2613 | SALTIRE | `*` |
| ☖ | `misc_2616` | U+2616 | WHITE SHOGI PIECE | `*` |
| ☗ | `misc_2617` | U+2617 | BLACK SHOGI PIECE | `*` |
| ☘ | `misc_2618` | U+2618 | SHAMROCK | `*` |
| ☙ | `misc_2619` | U+2619 | REVERSED ROTATED FLORAL HEART BULLET | `*` |
| ☚ | `misc_261A` | U+261A | BLACK LEFT POINTING INDEX | `*` |
| ☛ | `misc_261B` | U+261B | BLACK RIGHT POINTING INDEX | `*` |
| ☜ | `misc_261C` | U+261C | WHITE LEFT POINTING INDEX | `*` |
| ☝ | `misc_261D` | U+261D | WHITE UP POINTING INDEX | `*` |
| ☞ | `misc_261E` | U+261E | WHITE RIGHT POINTING INDEX | `*` |
| ☟ | `misc_261F` | U+261F | WHITE DOWN POINTING INDEX | `*` |
| ☠ | `misc_2620` | U+2620 | SKULL AND CROSSBONES | `*` |
| ☡ | `misc_2621` | U+2621 | CAUTION SIGN | `*` |
| ☢ | `misc_2622` | U+2622 | RADIOACTIVE SIGN | `*` |
| ☣ | `misc_2623` | U+2623 | BIOHAZARD SIGN | `*` |
| ☤ | `misc_2624` | U+2624 | CADUCEUS | `*` |
| ☥ | `misc_2625` | U+2625 | ANKH | `*` |
| ☦ | `misc_2626` | U+2626 | ORTHODOX CROSS | `*` |
| ☧ | `misc_2627` | U+2627 | CHI RHO | `*` |
| ☨ | `misc_2628` | U+2628 | CROSS OF LORRAINE | `*` |
| ☩ | `misc_2629` | U+2629 | CROSS OF JERUSALEM | `*` |
| ☪ | `misc_262A` | U+262A | STAR AND CRESCENT | `*` |
| ☫ | `misc_262B` | U+262B | FARSI SYMBOL | `*` |
| ☬ | `misc_262C` | U+262C | ADI SHAKTI | `*` |
| ☭ | `misc_262D` | U+262D | HAMMER AND SICKLE | `*` |
| ☮ | `misc_262E` | U+262E | PEACE SYMBOL | `*` |
| ☯ | `misc_262F` | U+262F | YIN YANG | `*` |
| ☸ | `misc_2638` | U+2638 | WHEEL OF DHARMA | `*` |
| ☹ | `misc_2639` | U+2639 | WHITE FROWNING FACE | `*` |
| ☺ | `misc_263A` | U+263A | WHITE SMILING FACE | `*` |
| ☻ | `misc_263B` | U+263B | BLACK SMILING FACE | `*` |
| ☼ | `misc_263C` | U+263C | WHITE SUN WITH RAYS | `*` |
| ☽ | `misc_263D` | U+263D | FIRST QUARTER MOON | `*` |
| ☾ | `misc_263E` | U+263E | LAST QUARTER MOON | `*` |
| ☿ | `misc_263F` | U+263F | MERCURY | `*` |
| ♀ | `misc_2640` | U+2640 | FEMALE SIGN | `*` |
| ♁ | `misc_2641` | U+2641 | EARTH | `*` |
| ♂ | `misc_2642` | U+2642 | MALE SIGN | `*` |
| ♃ | `misc_2643` | U+2643 | JUPITER | `*` |
| ♄ | `misc_2644` | U+2644 | SATURN | `*` |
| ♅ | `misc_2645` | U+2645 | URANUS | `*` |
| ♆ | `misc_2646` | U+2646 | NEPTUNE | `*` |
| ♇ | `misc_2647` | U+2647 | PLUTO | `*` |
| ♔ | `misc_2654` | U+2654 | WHITE CHESS KING | `*` |
| ♕ | `misc_2655` | U+2655 | WHITE CHESS QUEEN | `*` |
| ♖ | `misc_2656` | U+2656 | WHITE CHESS ROOK | `*` |
| ♗ | `misc_2657` | U+2657 | WHITE CHESS BISHOP | `*` |
| ♘ | `misc_2658` | U+2658 | WHITE CHESS KNIGHT | `*` |
| ♙ | `misc_2659` | U+2659 | WHITE CHESS PAWN | `*` |
| ♚ | `misc_265A` | U+265A | BLACK CHESS KING | `*` |
| ♛ | `misc_265B` | U+265B | BLACK CHESS QUEEN | `*` |
| ♜ | `misc_265C` | U+265C | BLACK CHESS ROOK | `*` |
| ♝ | `misc_265D` | U+265D | BLACK CHESS BISHOP | `*` |
| ♞ | `misc_265E` | U+265E | BLACK CHESS KNIGHT | `*` |
| ♟ | `misc_265F` | U+265F | BLACK CHESS PAWN | `*` |
| ♠ | `misc_2660` | U+2660 | BLACK SPADE SUIT | `*` |
| ♡ | `misc_2661` | U+2661 | WHITE HEART SUIT | `*` |
| ♢ | `misc_2662` | U+2662 | WHITE DIAMOND SUIT | `*` |
| ♣ | `misc_2663` | U+2663 | BLACK CLUB SUIT | `*` |
| ♤ | `misc_2664` | U+2664 | WHITE SPADE SUIT | `*` |
| ♥ | `misc_2665` | U+2665 | BLACK HEART SUIT | `*` |
| ♦ | `misc_2666` | U+2666 | BLACK DIAMOND SUIT | `*` |
| ♧ | `misc_2667` | U+2667 | WHITE CLUB SUIT | `*` |
| ♨ | `misc_2668` | U+2668 | HOT SPRINGS | `*` |
| ♩ | `misc_2669` | U+2669 | QUARTER NOTE | `*` |
| ♪ | `misc_266A` | U+266A | EIGHTH NOTE | `*` |
| ♫ | `misc_266B` | U+266B | BEAMED EIGHTH NOTES | `*` |
| ♬ | `misc_266C` | U+266C | BEAMED SIXTEENTH NOTES | `*` |
| ♭ | `misc_266D` | U+266D | MUSIC FLAT SIGN | `*` |
| ♮ | `misc_266E` | U+266E | MUSIC NATURAL SIGN | `*` |
| ♯ | `misc_266F` | U+266F | MUSIC SHARP SIGN | `*` |
| ♰ | `misc_2670` | U+2670 | WEST SYRIAC CROSS | `*` |
| ♱ | `misc_2671` | U+2671 | EAST SYRIAC CROSS | `*` |
| ♲ | `misc_2672` | U+2672 | UNIVERSAL RECYCLING SYMBOL | `*` |
| ♳ | `misc_2673` | U+2673 | RECYCLING SYMBOL FOR TYPE-1 PLASTICS | `*` |
| ♴ | `misc_2674` | U+2674 | RECYCLING SYMBOL FOR TYPE-2 PLASTICS | `*` |
| ♵ | `misc_2675` | U+2675 | RECYCLING SYMBOL FOR TYPE-3 PLASTICS | `*` |
| ♶ | `misc_2676` | U+2676 | RECYCLING SYMBOL FOR TYPE-4 PLASTICS | `*` |
| ♷ | `misc_2677` | U+2677 | RECYCLING SYMBOL FOR TYPE-5 PLASTICS | `*` |
| ♸ | `misc_2678` | U+2678 | RECYCLING SYMBOL FOR TYPE-6 PLASTICS | `*` |
| ♹ | `misc_2679` | U+2679 | RECYCLING SYMBOL FOR TYPE-7 PLASTICS | `*` |
| ♺ | `misc_267A` | U+267A | RECYCLING SYMBOL FOR GENERIC MATERIALS | `*` |
| ♻ | `misc_267B` | U+267B | BLACK UNIVERSAL RECYCLING SYMBOL | `*` |
| ♼ | `misc_267C` | U+267C | RECYCLED PAPER SYMBOL | `*` |
| ♽ | `misc_267D` | U+267D | PARTIALLY-RECYCLED PAPER SYMBOL | `*` |
| ♾ | `misc_267E` | U+267E | PERMANENT PAPER SIGN | `*` |
| ⚀ | `misc_2680` | U+2680 | DIE FACE-1 | `*` |
| ⚁ | `misc_2681` | U+2681 | DIE FACE-2 | `*` |
| ⚂ | `misc_2682` | U+2682 | DIE FACE-3 | `*` |
| ⚃ | `misc_2683` | U+2683 | DIE FACE-4 | `*` |
| ⚄ | `misc_2684` | U+2684 | DIE FACE-5 | `*` |
| ⚅ | `misc_2685` | U+2685 | DIE FACE-6 | `*` |
| ⚆ | `misc_2686` | U+2686 | WHITE CIRCLE WITH DOT RIGHT | `*` |
| ⚇ | `misc_2687` | U+2687 | WHITE CIRCLE WITH TWO DOTS | `*` |
| ⚈ | `misc_2688` | U+2688 | BLACK CIRCLE WITH WHITE DOT RIGHT | `*` |
| ⚉ | `misc_2689` | U+2689 | BLACK CIRCLE WITH TWO WHITE DOTS | `*` |
| ⚐ | `misc_2690` | U+2690 | WHITE FLAG | `*` |
| ⚑ | `misc_2691` | U+2691 | BLACK FLAG | `*` |
| ⚒ | `misc_2692` | U+2692 | HAMMER AND PICK | `*` |
| ⚔ | `misc_2694` | U+2694 | CROSSED SWORDS | `*` |
| ⚕ | `misc_2695` | U+2695 | STAFF OF AESCULAPIUS | `*` |
| ⚖ | `misc_2696` | U+2696 | SCALES | `*` |
| ⚗ | `misc_2697` | U+2697 | ALEMBIC | `*` |
| ⚘ | `misc_2698` | U+2698 | FLOWER | `*` |
| ⚙ | `misc_2699` | U+2699 | GEAR | `@` |
| ⚚ | `misc_269A` | U+269A | STAFF OF HERMES | `*` |
| ⚛ | `misc_269B` | U+269B | ATOM SYMBOL | `*` |
| ⚜ | `misc_269C` | U+269C | FLEUR-DE-LIS | `*` |
| ⚞ | `misc_269E` | U+269E | THREE LINES CONVERGING RIGHT | `*` |
| ⚟ | `misc_269F` | U+269F | THREE LINES CONVERGING LEFT | `*` |
| ⚠ | `misc_26A0` | U+26A0 | WARNING SIGN | `!` |
| ⚢ | `misc_26A2` | U+26A2 | DOUBLED FEMALE SIGN | `*` |
| ⚣ | `misc_26A3` | U+26A3 | DOUBLED MALE SIGN | `*` |
| ⚤ | `misc_26A4` | U+26A4 | INTERLOCKED FEMALE AND MALE SIGN | `*` |
| ⚥ | `misc_26A5` | U+26A5 | MALE AND FEMALE SIGN | `*` |
| ⚦ | `misc_26A6` | U+26A6 | MALE WITH STROKE SIGN | `*` |
| ⚧ | `misc_26A7` | U+26A7 | MALE WITH STROKE AND MALE AND FEMALE SIGN | `*` |
| ⚨ | `misc_26A8` | U+26A8 | VERTICAL MALE WITH STROKE SIGN | `*` |
| ⚩ | `misc_26A9` | U+26A9 | HORIZONTAL MALE WITH STROKE SIGN | `*` |
| ⚬ | `misc_26AC` | U+26AC | MEDIUM SMALL WHITE CIRCLE | `*` |
| ⚭ | `misc_26AD` | U+26AD | MARRIAGE SYMBOL | `*` |
| ⚮ | `misc_26AE` | U+26AE | DIVORCE SYMBOL | `*` |
| ⚯ | `misc_26AF` | U+26AF | UNMARRIED PARTNERSHIP SYMBOL | `*` |
| ⚰ | `misc_26B0` | U+26B0 | COFFIN | `*` |
| ⚱ | `misc_26B1` | U+26B1 | FUNERAL URN | `*` |
| ⚲ | `misc_26B2` | U+26B2 | NEUTER | `*` |
| ⚳ | `misc_26B3` | U+26B3 | CERES | `*` |
| ⚴ | `misc_26B4` | U+26B4 | PALLAS | `*` |
| ⚵ | `misc_26B5` | U+26B5 | JUNO | `*` |
| ⚶ | `misc_26B6` | U+26B6 | VESTA | `*` |
| ⚷ | `misc_26B7` | U+26B7 | CHIRON | `*` |
| ⚸ | `misc_26B8` | U+26B8 | BLACK MOON LILITH | `*` |
| ⛀ | `misc_26C0` | U+26C0 | WHITE DRAUGHTS MAN | `*` |
| ⛁ | `misc_26C1` | U+26C1 | WHITE DRAUGHTS KING | `*` |
| ⛂ | `misc_26C2` | U+26C2 | BLACK DRAUGHTS MAN | `*` |
| ⛃ | `misc_26C3` | U+26C3 | BLACK DRAUGHTS KING | `*` |
| ⛈ | `misc_26C8` | U+26C8 | THUNDER CLOUD AND RAIN | `*` |
| ⛏ | `misc_26CF` | U+26CF | PICK | `*` |
| ⛑ | `misc_26D1` | U+26D1 | HELMET WITH WHITE CROSS | `*` |
| ⛓ | `misc_26D3` | U+26D3 | CHAINS | `*` |
| ⛢ | `misc_26E2` | U+26E2 | ASTRONOMICAL SYMBOL FOR URANUS | `*` |
| ⛩ | `misc_26E9` | U+26E9 | SHINTO SHRINE | `*` |
| ⛰ | `misc_26F0` | U+26F0 | MOUNTAIN | `*` |
| ⛱ | `misc_26F1` | U+26F1 | UMBRELLA ON GROUND | `*` |
| ⛴ | `misc_26F4` | U+26F4 | FERRY | `*` |
| ⛷ | `misc_26F7` | U+26F7 | SKIER | `*` |
| ⛸ | `misc_26F8` | U+26F8 | ICE SKATE | `*` |
| ⛹ | `misc_26F9` | U+26F9 | PERSON WITH BALL | `*` |

## dingbats

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ✁ | `dingbat_2701` | U+2701 | UPPER BLADE SCISSORS | `*` |
| ✂ | `dingbat_2702` | U+2702 | BLACK SCISSORS | `X` |
| ✃ | `dingbat_2703` | U+2703 | LOWER BLADE SCISSORS | `*` |
| ✄ | `dingbat_2704` | U+2704 | WHITE SCISSORS | `*` |
| ✆ | `dingbat_2706` | U+2706 | TELEPHONE LOCATION SIGN | `*` |
| ✇ | `dingbat_2707` | U+2707 | TAPE DRIVE | `*` |
| ✈ | `dingbat_2708` | U+2708 | AIRPLANE | `*` |
| ✉ | `dingbat_2709` | U+2709 | ENVELOPE | `*` |
| ✌ | `dingbat_270C` | U+270C | VICTORY HAND | `*` |
| ✍ | `dingbat_270D` | U+270D | WRITING HAND | `*` |
| ✎ | `dingbat_270E` | U+270E | LOWER RIGHT PENCIL | `*` |
| ✏ | `dingbat_270F` | U+270F | PENCIL | `*` |
| ✐ | `dingbat_2710` | U+2710 | UPPER RIGHT PENCIL | `*` |
| ✑ | `dingbat_2711` | U+2711 | WHITE NIB | `*` |
| ✒ | `dingbat_2712` | U+2712 | BLACK NIB | `*` |
| ✓ | `dingbat_2713` | U+2713 | CHECK MARK | `v` |
| ✔ | `dingbat_2714` | U+2714 | HEAVY CHECK MARK | `v` |
| ✕ | `dingbat_2715` | U+2715 | MULTIPLICATION X | `x` |
| ✖ | `dingbat_2716` | U+2716 | HEAVY MULTIPLICATION X | `x` |
| ✗ | `dingbat_2717` | U+2717 | BALLOT X | `x` |
| ✘ | `dingbat_2718` | U+2718 | HEAVY BALLOT X | `x` |
| ✙ | `dingbat_2719` | U+2719 | OUTLINED GREEK CROSS | `*` |
| ✚ | `dingbat_271A` | U+271A | HEAVY GREEK CROSS | `*` |
| ✛ | `dingbat_271B` | U+271B | OPEN CENTRE CROSS | `*` |
| ✜ | `dingbat_271C` | U+271C | HEAVY OPEN CENTRE CROSS | `*` |
| ✝ | `dingbat_271D` | U+271D | LATIN CROSS | `*` |
| ✞ | `dingbat_271E` | U+271E | SHADOWED WHITE LATIN CROSS | `*` |
| ✟ | `dingbat_271F` | U+271F | OUTLINED LATIN CROSS | `*` |
| ✠ | `dingbat_2720` | U+2720 | MALTESE CROSS | `*` |
| ✡ | `dingbat_2721` | U+2721 | STAR OF DAVID | `*` |
| ✢ | `dingbat_2722` | U+2722 | FOUR TEARDROP-SPOKED ASTERISK | `*` |
| ✣ | `dingbat_2723` | U+2723 | FOUR BALLOON-SPOKED ASTERISK | `*` |
| ✤ | `dingbat_2724` | U+2724 | HEAVY FOUR BALLOON-SPOKED ASTERISK | `*` |
| ✥ | `dingbat_2725` | U+2725 | FOUR CLUB-SPOKED ASTERISK | `*` |
| ✦ | `dingbat_2726` | U+2726 | BLACK FOUR POINTED STAR | `*` |
| ✧ | `dingbat_2727` | U+2727 | WHITE FOUR POINTED STAR | `*` |
| ✩ | `dingbat_2729` | U+2729 | STRESS OUTLINED WHITE STAR | `*` |
| ✪ | `dingbat_272A` | U+272A | CIRCLED WHITE STAR | `*` |
| ✫ | `dingbat_272B` | U+272B | OPEN CENTRE BLACK STAR | `*` |
| ✬ | `dingbat_272C` | U+272C | BLACK CENTRE WHITE STAR | `*` |
| ✭ | `dingbat_272D` | U+272D | OUTLINED BLACK STAR | `*` |
| ✮ | `dingbat_272E` | U+272E | HEAVY OUTLINED BLACK STAR | `*` |
| ✯ | `dingbat_272F` | U+272F | PINWHEEL STAR | `*` |
| ✰ | `dingbat_2730` | U+2730 | SHADOWED WHITE STAR | `*` |
| ✱ | `dingbat_2731` | U+2731 | HEAVY ASTERISK | `*` |
| ✲ | `dingbat_2732` | U+2732 | OPEN CENTRE ASTERISK | `*` |
| ✳ | `dingbat_2733` | U+2733 | EIGHT SPOKED ASTERISK | `*` |
| ✴ | `dingbat_2734` | U+2734 | EIGHT POINTED BLACK STAR | `*` |
| ✵ | `dingbat_2735` | U+2735 | EIGHT POINTED PINWHEEL STAR | `*` |
| ✶ | `dingbat_2736` | U+2736 | SIX POINTED BLACK STAR | `*` |
| ✷ | `dingbat_2737` | U+2737 | EIGHT POINTED RECTILINEAR BLACK STAR | `*` |
| ✸ | `dingbat_2738` | U+2738 | HEAVY EIGHT POINTED RECTILINEAR BLACK STAR | `*` |
| ✹ | `dingbat_2739` | U+2739 | TWELVE POINTED BLACK STAR | `*` |
| ✺ | `dingbat_273A` | U+273A | SIXTEEN POINTED ASTERISK | `*` |
| ✻ | `dingbat_273B` | U+273B | TEARDROP-SPOKED ASTERISK | `*` |
| ✼ | `dingbat_273C` | U+273C | OPEN CENTRE TEARDROP-SPOKED ASTERISK | `*` |
| ✽ | `dingbat_273D` | U+273D | HEAVY TEARDROP-SPOKED ASTERISK | `*` |
| ✾ | `dingbat_273E` | U+273E | SIX PETALLED BLACK AND WHITE FLORETTE | `*` |
| ✿ | `dingbat_273F` | U+273F | BLACK FLORETTE | `*` |
| ❀ | `dingbat_2740` | U+2740 | WHITE FLORETTE | `*` |
| ❁ | `dingbat_2741` | U+2741 | EIGHT PETALLED OUTLINED BLACK FLORETTE | `*` |
| ❂ | `dingbat_2742` | U+2742 | CIRCLED OPEN CENTRE EIGHT POINTED STAR | `*` |
| ❃ | `dingbat_2743` | U+2743 | HEAVY TEARDROP-SPOKED PINWHEEL ASTERISK | `*` |
| ❄ | `dingbat_2744` | U+2744 | SNOWFLAKE | `*` |
| ❅ | `dingbat_2745` | U+2745 | TIGHT TRIFOLIATE SNOWFLAKE | `*` |
| ❆ | `dingbat_2746` | U+2746 | HEAVY CHEVRON SNOWFLAKE | `*` |
| ❇ | `dingbat_2747` | U+2747 | SPARKLE | `*` |
| ❈ | `dingbat_2748` | U+2748 | HEAVY SPARKLE | `*` |
| ❉ | `dingbat_2749` | U+2749 | BALLOON-SPOKED ASTERISK | `*` |
| ❊ | `dingbat_274A` | U+274A | EIGHT TEARDROP-SPOKED PROPELLER ASTERISK | `*` |
| ❋ | `dingbat_274B` | U+274B | HEAVY EIGHT TEARDROP-SPOKED PROPELLER ASTERISK | `*` |
| ❍ | `dingbat_274D` | U+274D | SHADOWED WHITE CIRCLE | `*` |
| ❏ | `dingbat_274F` | U+274F | LOWER RIGHT DROP-SHADOWED WHITE SQUARE | `*` |
| ❐ | `dingbat_2750` | U+2750 | UPPER RIGHT DROP-SHADOWED WHITE SQUARE | `*` |
| ❒ | `dingbat_2752` | U+2752 | UPPER RIGHT SHADOWED WHITE SQUARE | `*` |
| ❖ | `dingbat_2756` | U+2756 | BLACK DIAMOND MINUS WHITE X | `*` |
| ❘ | `dingbat_2758` | U+2758 | LIGHT VERTICAL BAR | `*` |
| ❙ | `dingbat_2759` | U+2759 | MEDIUM VERTICAL BAR | `*` |
| ❚ | `dingbat_275A` | U+275A | HEAVY VERTICAL BAR | `*` |
| ❛ | `dingbat_275B` | U+275B | HEAVY SINGLE TURNED COMMA QUOTATION MARK ORNAMENT | `*` |
| ❜ | `dingbat_275C` | U+275C | HEAVY SINGLE COMMA QUOTATION MARK ORNAMENT | `*` |
| ❝ | `dingbat_275D` | U+275D | HEAVY DOUBLE TURNED COMMA QUOTATION MARK ORNAMENT | `*` |
| ❞ | `dingbat_275E` | U+275E | HEAVY DOUBLE COMMA QUOTATION MARK ORNAMENT | `*` |
| ❡ | `dingbat_2761` | U+2761 | CURVED STEM PARAGRAPH SIGN ORNAMENT | `*` |
| ❢ | `dingbat_2762` | U+2762 | HEAVY EXCLAMATION MARK ORNAMENT | `*` |
| ❣ | `dingbat_2763` | U+2763 | HEAVY HEART EXCLAMATION MARK ORNAMENT | `*` |
| ❤ | `dingbat_2764` | U+2764 | HEAVY BLACK HEART | `*` |
| ❥ | `dingbat_2765` | U+2765 | ROTATED HEAVY BLACK HEART BULLET | `*` |
| ❦ | `dingbat_2766` | U+2766 | FLORAL HEART | `*` |
| ❧ | `dingbat_2767` | U+2767 | ROTATED FLORAL HEART BULLET | `*` |
| ❨ | `dingbat_2768` | U+2768 | MEDIUM LEFT PARENTHESIS ORNAMENT | `*` |
| ❩ | `dingbat_2769` | U+2769 | MEDIUM RIGHT PARENTHESIS ORNAMENT | `*` |
| ❪ | `dingbat_276A` | U+276A | MEDIUM FLATTENED LEFT PARENTHESIS ORNAMENT | `*` |
| ❫ | `dingbat_276B` | U+276B | MEDIUM FLATTENED RIGHT PARENTHESIS ORNAMENT | `*` |
| ❬ | `dingbat_276C` | U+276C | MEDIUM LEFT-POINTING ANGLE BRACKET ORNAMENT | `*` |
| ❭ | `dingbat_276D` | U+276D | MEDIUM RIGHT-POINTING ANGLE BRACKET ORNAMENT | `*` |
| ❮ | `dingbat_276E` | U+276E | HEAVY LEFT-POINTING ANGLE QUOTATION MARK ORNAMENT | `*` |
| ❯ | `dingbat_276F` | U+276F | HEAVY RIGHT-POINTING ANGLE QUOTATION MARK ORNAMENT | `*` |
| ❰ | `dingbat_2770` | U+2770 | HEAVY LEFT-POINTING ANGLE BRACKET ORNAMENT | `*` |
| ❱ | `dingbat_2771` | U+2771 | HEAVY RIGHT-POINTING ANGLE BRACKET ORNAMENT | `*` |
| ❲ | `dingbat_2772` | U+2772 | LIGHT LEFT TORTOISE SHELL BRACKET ORNAMENT | `*` |
| ❳ | `dingbat_2773` | U+2773 | LIGHT RIGHT TORTOISE SHELL BRACKET ORNAMENT | `*` |
| ❴ | `dingbat_2774` | U+2774 | MEDIUM LEFT CURLY BRACKET ORNAMENT | `*` |
| ❵ | `dingbat_2775` | U+2775 | MEDIUM RIGHT CURLY BRACKET ORNAMENT | `*` |
| ❶ | `dingbat_2776` | U+2776 | DINGBAT NEGATIVE CIRCLED DIGIT ONE | `*` |
| ❷ | `dingbat_2777` | U+2777 | DINGBAT NEGATIVE CIRCLED DIGIT TWO | `*` |
| ❸ | `dingbat_2778` | U+2778 | DINGBAT NEGATIVE CIRCLED DIGIT THREE | `*` |
| ❹ | `dingbat_2779` | U+2779 | DINGBAT NEGATIVE CIRCLED DIGIT FOUR | `*` |
| ❺ | `dingbat_277A` | U+277A | DINGBAT NEGATIVE CIRCLED DIGIT FIVE | `*` |
| ❻ | `dingbat_277B` | U+277B | DINGBAT NEGATIVE CIRCLED DIGIT SIX | `*` |
| ❼ | `dingbat_277C` | U+277C | DINGBAT NEGATIVE CIRCLED DIGIT SEVEN | `*` |
| ❽ | `dingbat_277D` | U+277D | DINGBAT NEGATIVE CIRCLED DIGIT EIGHT | `*` |
| ❾ | `dingbat_277E` | U+277E | DINGBAT NEGATIVE CIRCLED DIGIT NINE | `*` |
| ❿ | `dingbat_277F` | U+277F | DINGBAT NEGATIVE CIRCLED NUMBER TEN | `*` |
| ➀ | `dingbat_2780` | U+2780 | DINGBAT CIRCLED SANS-SERIF DIGIT ONE | `*` |
| ➁ | `dingbat_2781` | U+2781 | DINGBAT CIRCLED SANS-SERIF DIGIT TWO | `*` |
| ➂ | `dingbat_2782` | U+2782 | DINGBAT CIRCLED SANS-SERIF DIGIT THREE | `*` |
| ➃ | `dingbat_2783` | U+2783 | DINGBAT CIRCLED SANS-SERIF DIGIT FOUR | `*` |
| ➄ | `dingbat_2784` | U+2784 | DINGBAT CIRCLED SANS-SERIF DIGIT FIVE | `*` |
| ➅ | `dingbat_2785` | U+2785 | DINGBAT CIRCLED SANS-SERIF DIGIT SIX | `*` |
| ➆ | `dingbat_2786` | U+2786 | DINGBAT CIRCLED SANS-SERIF DIGIT SEVEN | `*` |
| ➇ | `dingbat_2787` | U+2787 | DINGBAT CIRCLED SANS-SERIF DIGIT EIGHT | `*` |
| ➈ | `dingbat_2788` | U+2788 | DINGBAT CIRCLED SANS-SERIF DIGIT NINE | `*` |
| ➉ | `dingbat_2789` | U+2789 | DINGBAT CIRCLED SANS-SERIF NUMBER TEN | `*` |
| ➊ | `dingbat_278A` | U+278A | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT ONE | `*` |
| ➋ | `dingbat_278B` | U+278B | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT TWO | `*` |
| ➌ | `dingbat_278C` | U+278C | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT THREE | `*` |
| ➍ | `dingbat_278D` | U+278D | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT FOUR | `*` |
| ➎ | `dingbat_278E` | U+278E | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT FIVE | `*` |
| ➏ | `dingbat_278F` | U+278F | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT SIX | `*` |
| ➐ | `dingbat_2790` | U+2790 | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT SEVEN | `*` |
| ➑ | `dingbat_2791` | U+2791 | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT EIGHT | `*` |
| ➒ | `dingbat_2792` | U+2792 | DINGBAT NEGATIVE CIRCLED SANS-SERIF DIGIT NINE | `*` |
| ➓ | `dingbat_2793` | U+2793 | DINGBAT NEGATIVE CIRCLED SANS-SERIF NUMBER TEN | `*` |
| ➔ | `dingbat_2794` | U+2794 | HEAVY WIDE-HEADED RIGHTWARDS ARROW | `>` |
| ➘ | `dingbat_2798` | U+2798 | HEAVY SOUTH EAST ARROW | `*` |
| ➙ | `dingbat_2799` | U+2799 | HEAVY RIGHTWARDS ARROW | `*` |
| ➚ | `dingbat_279A` | U+279A | HEAVY NORTH EAST ARROW | `*` |
| ➛ | `dingbat_279B` | U+279B | DRAFTING POINT RIGHTWARDS ARROW | `*` |
| ➜ | `dingbat_279C` | U+279C | HEAVY ROUND-TIPPED RIGHTWARDS ARROW | `*` |
| ➝ | `dingbat_279D` | U+279D | TRIANGLE-HEADED RIGHTWARDS ARROW | `*` |
| ➞ | `dingbat_279E` | U+279E | HEAVY TRIANGLE-HEADED RIGHTWARDS ARROW | `*` |
| ➟ | `dingbat_279F` | U+279F | DASHED TRIANGLE-HEADED RIGHTWARDS ARROW | `*` |
| ➠ | `dingbat_27A0` | U+27A0 | HEAVY DASHED TRIANGLE-HEADED RIGHTWARDS ARROW | `*` |
| ➡ | `dingbat_27A1` | U+27A1 | BLACK RIGHTWARDS ARROW | `*` |
| ➢ | `dingbat_27A2` | U+27A2 | THREE-D TOP-LIGHTED RIGHTWARDS ARROWHEAD | `>` |
| ➣ | `dingbat_27A3` | U+27A3 | THREE-D BOTTOM-LIGHTED RIGHTWARDS ARROWHEAD | `*` |
| ➤ | `dingbat_27A4` | U+27A4 | BLACK RIGHTWARDS ARROWHEAD | `*` |
| ➥ | `dingbat_27A5` | U+27A5 | HEAVY BLACK CURVED DOWNWARDS AND RIGHTWARDS ARROW | `*` |
| ➦ | `dingbat_27A6` | U+27A6 | HEAVY BLACK CURVED UPWARDS AND RIGHTWARDS ARROW | `*` |
| ➧ | `dingbat_27A7` | U+27A7 | SQUAT BLACK RIGHTWARDS ARROW | `*` |
| ➨ | `dingbat_27A8` | U+27A8 | HEAVY CONCAVE-POINTED BLACK RIGHTWARDS ARROW | `*` |
| ➩ | `dingbat_27A9` | U+27A9 | RIGHT-SHADED WHITE RIGHTWARDS ARROW | `*` |
| ➪ | `dingbat_27AA` | U+27AA | LEFT-SHADED WHITE RIGHTWARDS ARROW | `*` |
| ➫ | `dingbat_27AB` | U+27AB | BACK-TILTED SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➬ | `dingbat_27AC` | U+27AC | FRONT-TILTED SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➭ | `dingbat_27AD` | U+27AD | HEAVY LOWER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➮ | `dingbat_27AE` | U+27AE | HEAVY UPPER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➯ | `dingbat_27AF` | U+27AF | NOTCHED LOWER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➱ | `dingbat_27B1` | U+27B1 | NOTCHED UPPER RIGHT-SHADOWED WHITE RIGHTWARDS ARROW | `*` |
| ➲ | `dingbat_27B2` | U+27B2 | CIRCLED HEAVY WHITE RIGHTWARDS ARROW | `*` |
| ➳ | `dingbat_27B3` | U+27B3 | WHITE-FEATHERED RIGHTWARDS ARROW | `*` |
| ➴ | `dingbat_27B4` | U+27B4 | BLACK-FEATHERED SOUTH EAST ARROW | `*` |
| ➵ | `dingbat_27B5` | U+27B5 | BLACK-FEATHERED RIGHTWARDS ARROW | `*` |
| ➶ | `dingbat_27B6` | U+27B6 | BLACK-FEATHERED NORTH EAST ARROW | `*` |
| ➷ | `dingbat_27B7` | U+27B7 | HEAVY BLACK-FEATHERED SOUTH EAST ARROW | `*` |
| ➸ | `dingbat_27B8` | U+27B8 | HEAVY BLACK-FEATHERED RIGHTWARDS ARROW | `*` |
| ➹ | `dingbat_27B9` | U+27B9 | HEAVY BLACK-FEATHERED NORTH EAST ARROW | `*` |
| ➺ | `dingbat_27BA` | U+27BA | TEARDROP-BARBED RIGHTWARDS ARROW | `*` |
| ➻ | `dingbat_27BB` | U+27BB | HEAVY TEARDROP-SHANKED RIGHTWARDS ARROW | `*` |
| ➼ | `dingbat_27BC` | U+27BC | WEDGE-TAILED RIGHTWARDS ARROW | `*` |
| ➽ | `dingbat_27BD` | U+27BD | HEAVY WEDGE-TAILED RIGHTWARDS ARROW | `*` |
| ➾ | `dingbat_27BE` | U+27BE | OPEN-OUTLINED RIGHTWARDS ARROW | `*` |

## braille

| Glyph | Id | Codepoint | Unicode name | Fallback |
|---|---|---|---|---|
| ⠁ | `braille_2801` | U+2801 | BRAILLE PATTERN DOTS-1 | `*` |
| ⠂ | `braille_2802` | U+2802 | BRAILLE PATTERN DOTS-2 | `*` |
| ⠃ | `braille_2803` | U+2803 | BRAILLE PATTERN DOTS-12 | `*` |
| ⠄ | `braille_2804` | U+2804 | BRAILLE PATTERN DOTS-3 | `*` |
| ⠅ | `braille_2805` | U+2805 | BRAILLE PATTERN DOTS-13 | `*` |
| ⠆ | `braille_2806` | U+2806 | BRAILLE PATTERN DOTS-23 | `*` |
| ⠇ | `braille_2807` | U+2807 | BRAILLE PATTERN DOTS-123 | `*` |
| ⠈ | `braille_2808` | U+2808 | BRAILLE PATTERN DOTS-4 | `*` |
| ⠉ | `braille_2809` | U+2809 | BRAILLE PATTERN DOTS-14 | `*` |
| ⠊ | `braille_280A` | U+280A | BRAILLE PATTERN DOTS-24 | `*` |
| ⠋ | `braille_280B` | U+280B | BRAILLE PATTERN DOTS-124 | `*` |
| ⠌ | `braille_280C` | U+280C | BRAILLE PATTERN DOTS-34 | `*` |
| ⠍ | `braille_280D` | U+280D | BRAILLE PATTERN DOTS-134 | `*` |
| ⠎ | `braille_280E` | U+280E | BRAILLE PATTERN DOTS-234 | `*` |
| ⠏ | `braille_280F` | U+280F | BRAILLE PATTERN DOTS-1234 | `*` |
| ⠐ | `braille_2810` | U+2810 | BRAILLE PATTERN DOTS-5 | `*` |
| ⠑ | `braille_2811` | U+2811 | BRAILLE PATTERN DOTS-15 | `*` |
| ⠒ | `braille_2812` | U+2812 | BRAILLE PATTERN DOTS-25 | `*` |
| ⠓ | `braille_2813` | U+2813 | BRAILLE PATTERN DOTS-125 | `*` |
| ⠔ | `braille_2814` | U+2814 | BRAILLE PATTERN DOTS-35 | `*` |
| ⠕ | `braille_2815` | U+2815 | BRAILLE PATTERN DOTS-135 | `*` |
| ⠖ | `braille_2816` | U+2816 | BRAILLE PATTERN DOTS-235 | `*` |
| ⠗ | `braille_2817` | U+2817 | BRAILLE PATTERN DOTS-1235 | `*` |
| ⠘ | `braille_2818` | U+2818 | BRAILLE PATTERN DOTS-45 | `*` |
| ⠙ | `braille_2819` | U+2819 | BRAILLE PATTERN DOTS-145 | `*` |
| ⠚ | `braille_281A` | U+281A | BRAILLE PATTERN DOTS-245 | `*` |
| ⠛ | `braille_281B` | U+281B | BRAILLE PATTERN DOTS-1245 | `*` |
| ⠜ | `braille_281C` | U+281C | BRAILLE PATTERN DOTS-345 | `*` |
| ⠝ | `braille_281D` | U+281D | BRAILLE PATTERN DOTS-1345 | `*` |
| ⠞ | `braille_281E` | U+281E | BRAILLE PATTERN DOTS-2345 | `*` |
| ⠟ | `braille_281F` | U+281F | BRAILLE PATTERN DOTS-12345 | `*` |
| ⠠ | `braille_2820` | U+2820 | BRAILLE PATTERN DOTS-6 | `*` |
| ⠡ | `braille_2821` | U+2821 | BRAILLE PATTERN DOTS-16 | `*` |
| ⠢ | `braille_2822` | U+2822 | BRAILLE PATTERN DOTS-26 | `*` |
| ⠣ | `braille_2823` | U+2823 | BRAILLE PATTERN DOTS-126 | `*` |
| ⠤ | `braille_2824` | U+2824 | BRAILLE PATTERN DOTS-36 | `*` |
| ⠥ | `braille_2825` | U+2825 | BRAILLE PATTERN DOTS-136 | `*` |
| ⠦ | `braille_2826` | U+2826 | BRAILLE PATTERN DOTS-236 | `*` |
| ⠧ | `braille_2827` | U+2827 | BRAILLE PATTERN DOTS-1236 | `*` |
| ⠨ | `braille_2828` | U+2828 | BRAILLE PATTERN DOTS-46 | `*` |
| ⠩ | `braille_2829` | U+2829 | BRAILLE PATTERN DOTS-146 | `*` |
| ⠪ | `braille_282A` | U+282A | BRAILLE PATTERN DOTS-246 | `*` |
| ⠫ | `braille_282B` | U+282B | BRAILLE PATTERN DOTS-1246 | `*` |
| ⠬ | `braille_282C` | U+282C | BRAILLE PATTERN DOTS-346 | `*` |
| ⠭ | `braille_282D` | U+282D | BRAILLE PATTERN DOTS-1346 | `*` |
| ⠮ | `braille_282E` | U+282E | BRAILLE PATTERN DOTS-2346 | `*` |
| ⠯ | `braille_282F` | U+282F | BRAILLE PATTERN DOTS-12346 | `*` |
| ⠰ | `braille_2830` | U+2830 | BRAILLE PATTERN DOTS-56 | `*` |
| ⠱ | `braille_2831` | U+2831 | BRAILLE PATTERN DOTS-156 | `*` |
| ⠲ | `braille_2832` | U+2832 | BRAILLE PATTERN DOTS-256 | `*` |
| ⠳ | `braille_2833` | U+2833 | BRAILLE PATTERN DOTS-1256 | `*` |
| ⠴ | `braille_2834` | U+2834 | BRAILLE PATTERN DOTS-356 | `*` |
| ⠵ | `braille_2835` | U+2835 | BRAILLE PATTERN DOTS-1356 | `*` |
| ⠶ | `braille_2836` | U+2836 | BRAILLE PATTERN DOTS-2356 | `*` |
| ⠷ | `braille_2837` | U+2837 | BRAILLE PATTERN DOTS-12356 | `*` |
| ⠸ | `braille_2838` | U+2838 | BRAILLE PATTERN DOTS-456 | `*` |
| ⠹ | `braille_2839` | U+2839 | BRAILLE PATTERN DOTS-1456 | `*` |
| ⠺ | `braille_283A` | U+283A | BRAILLE PATTERN DOTS-2456 | `*` |
| ⠻ | `braille_283B` | U+283B | BRAILLE PATTERN DOTS-12456 | `*` |
| ⠼ | `braille_283C` | U+283C | BRAILLE PATTERN DOTS-3456 | `*` |
| ⠽ | `braille_283D` | U+283D | BRAILLE PATTERN DOTS-13456 | `*` |
| ⠾ | `braille_283E` | U+283E | BRAILLE PATTERN DOTS-23456 | `*` |
| ⠿ | `braille_283F` | U+283F | BRAILLE PATTERN DOTS-123456 | `*` |
| ⡀ | `braille_2840` | U+2840 | BRAILLE PATTERN DOTS-7 | `*` |
| ⡁ | `braille_2841` | U+2841 | BRAILLE PATTERN DOTS-17 | `*` |
| ⡂ | `braille_2842` | U+2842 | BRAILLE PATTERN DOTS-27 | `*` |
| ⡃ | `braille_2843` | U+2843 | BRAILLE PATTERN DOTS-127 | `*` |
| ⡄ | `braille_2844` | U+2844 | BRAILLE PATTERN DOTS-37 | `*` |
| ⡅ | `braille_2845` | U+2845 | BRAILLE PATTERN DOTS-137 | `*` |
| ⡆ | `braille_2846` | U+2846 | BRAILLE PATTERN DOTS-237 | `*` |
| ⡇ | `braille_2847` | U+2847 | BRAILLE PATTERN DOTS-1237 | `*` |
| ⡈ | `braille_2848` | U+2848 | BRAILLE PATTERN DOTS-47 | `*` |
| ⡉ | `braille_2849` | U+2849 | BRAILLE PATTERN DOTS-147 | `*` |
| ⡊ | `braille_284A` | U+284A | BRAILLE PATTERN DOTS-247 | `*` |
| ⡋ | `braille_284B` | U+284B | BRAILLE PATTERN DOTS-1247 | `*` |
| ⡌ | `braille_284C` | U+284C | BRAILLE PATTERN DOTS-347 | `*` |
| ⡍ | `braille_284D` | U+284D | BRAILLE PATTERN DOTS-1347 | `*` |
| ⡎ | `braille_284E` | U+284E | BRAILLE PATTERN DOTS-2347 | `*` |
| ⡏ | `braille_284F` | U+284F | BRAILLE PATTERN DOTS-12347 | `*` |
| ⡐ | `braille_2850` | U+2850 | BRAILLE PATTERN DOTS-57 | `*` |
| ⡑ | `braille_2851` | U+2851 | BRAILLE PATTERN DOTS-157 | `*` |
| ⡒ | `braille_2852` | U+2852 | BRAILLE PATTERN DOTS-257 | `*` |
| ⡓ | `braille_2853` | U+2853 | BRAILLE PATTERN DOTS-1257 | `*` |
| ⡔ | `braille_2854` | U+2854 | BRAILLE PATTERN DOTS-357 | `*` |
| ⡕ | `braille_2855` | U+2855 | BRAILLE PATTERN DOTS-1357 | `*` |
| ⡖ | `braille_2856` | U+2856 | BRAILLE PATTERN DOTS-2357 | `*` |
| ⡗ | `braille_2857` | U+2857 | BRAILLE PATTERN DOTS-12357 | `*` |
| ⡘ | `braille_2858` | U+2858 | BRAILLE PATTERN DOTS-457 | `*` |
| ⡙ | `braille_2859` | U+2859 | BRAILLE PATTERN DOTS-1457 | `*` |
| ⡚ | `braille_285A` | U+285A | BRAILLE PATTERN DOTS-2457 | `*` |
| ⡛ | `braille_285B` | U+285B | BRAILLE PATTERN DOTS-12457 | `*` |
| ⡜ | `braille_285C` | U+285C | BRAILLE PATTERN DOTS-3457 | `*` |
| ⡝ | `braille_285D` | U+285D | BRAILLE PATTERN DOTS-13457 | `*` |
| ⡞ | `braille_285E` | U+285E | BRAILLE PATTERN DOTS-23457 | `*` |
| ⡟ | `braille_285F` | U+285F | BRAILLE PATTERN DOTS-123457 | `*` |
| ⡠ | `braille_2860` | U+2860 | BRAILLE PATTERN DOTS-67 | `*` |
| ⡡ | `braille_2861` | U+2861 | BRAILLE PATTERN DOTS-167 | `*` |
| ⡢ | `braille_2862` | U+2862 | BRAILLE PATTERN DOTS-267 | `*` |
| ⡣ | `braille_2863` | U+2863 | BRAILLE PATTERN DOTS-1267 | `*` |
| ⡤ | `braille_2864` | U+2864 | BRAILLE PATTERN DOTS-367 | `*` |
| ⡥ | `braille_2865` | U+2865 | BRAILLE PATTERN DOTS-1367 | `*` |
| ⡦ | `braille_2866` | U+2866 | BRAILLE PATTERN DOTS-2367 | `*` |
| ⡧ | `braille_2867` | U+2867 | BRAILLE PATTERN DOTS-12367 | `*` |
| ⡨ | `braille_2868` | U+2868 | BRAILLE PATTERN DOTS-467 | `*` |
| ⡩ | `braille_2869` | U+2869 | BRAILLE PATTERN DOTS-1467 | `*` |
| ⡪ | `braille_286A` | U+286A | BRAILLE PATTERN DOTS-2467 | `*` |
| ⡫ | `braille_286B` | U+286B | BRAILLE PATTERN DOTS-12467 | `*` |
| ⡬ | `braille_286C` | U+286C | BRAILLE PATTERN DOTS-3467 | `*` |
| ⡭ | `braille_286D` | U+286D | BRAILLE PATTERN DOTS-13467 | `*` |
| ⡮ | `braille_286E` | U+286E | BRAILLE PATTERN DOTS-23467 | `*` |
| ⡯ | `braille_286F` | U+286F | BRAILLE PATTERN DOTS-123467 | `*` |
| ⡰ | `braille_2870` | U+2870 | BRAILLE PATTERN DOTS-567 | `*` |
| ⡱ | `braille_2871` | U+2871 | BRAILLE PATTERN DOTS-1567 | `*` |
| ⡲ | `braille_2872` | U+2872 | BRAILLE PATTERN DOTS-2567 | `*` |
| ⡳ | `braille_2873` | U+2873 | BRAILLE PATTERN DOTS-12567 | `*` |
| ⡴ | `braille_2874` | U+2874 | BRAILLE PATTERN DOTS-3567 | `*` |
| ⡵ | `braille_2875` | U+2875 | BRAILLE PATTERN DOTS-13567 | `*` |
| ⡶ | `braille_2876` | U+2876 | BRAILLE PATTERN DOTS-23567 | `*` |
| ⡷ | `braille_2877` | U+2877 | BRAILLE PATTERN DOTS-123567 | `*` |
| ⡸ | `braille_2878` | U+2878 | BRAILLE PATTERN DOTS-4567 | `*` |
| ⡹ | `braille_2879` | U+2879 | BRAILLE PATTERN DOTS-14567 | `*` |
| ⡺ | `braille_287A` | U+287A | BRAILLE PATTERN DOTS-24567 | `*` |
| ⡻ | `braille_287B` | U+287B | BRAILLE PATTERN DOTS-124567 | `*` |
| ⡼ | `braille_287C` | U+287C | BRAILLE PATTERN DOTS-34567 | `*` |
| ⡽ | `braille_287D` | U+287D | BRAILLE PATTERN DOTS-134567 | `*` |
| ⡾ | `braille_287E` | U+287E | BRAILLE PATTERN DOTS-234567 | `*` |
| ⡿ | `braille_287F` | U+287F | BRAILLE PATTERN DOTS-1234567 | `*` |
| ⢀ | `braille_2880` | U+2880 | BRAILLE PATTERN DOTS-8 | `*` |
| ⢁ | `braille_2881` | U+2881 | BRAILLE PATTERN DOTS-18 | `*` |
| ⢂ | `braille_2882` | U+2882 | BRAILLE PATTERN DOTS-28 | `*` |
| ⢃ | `braille_2883` | U+2883 | BRAILLE PATTERN DOTS-128 | `*` |
| ⢄ | `braille_2884` | U+2884 | BRAILLE PATTERN DOTS-38 | `*` |
| ⢅ | `braille_2885` | U+2885 | BRAILLE PATTERN DOTS-138 | `*` |
| ⢆ | `braille_2886` | U+2886 | BRAILLE PATTERN DOTS-238 | `*` |
| ⢇ | `braille_2887` | U+2887 | BRAILLE PATTERN DOTS-1238 | `*` |
| ⢈ | `braille_2888` | U+2888 | BRAILLE PATTERN DOTS-48 | `*` |
| ⢉ | `braille_2889` | U+2889 | BRAILLE PATTERN DOTS-148 | `*` |
| ⢊ | `braille_288A` | U+288A | BRAILLE PATTERN DOTS-248 | `*` |
| ⢋ | `braille_288B` | U+288B | BRAILLE PATTERN DOTS-1248 | `*` |
| ⢌ | `braille_288C` | U+288C | BRAILLE PATTERN DOTS-348 | `*` |
| ⢍ | `braille_288D` | U+288D | BRAILLE PATTERN DOTS-1348 | `*` |
| ⢎ | `braille_288E` | U+288E | BRAILLE PATTERN DOTS-2348 | `*` |
| ⢏ | `braille_288F` | U+288F | BRAILLE PATTERN DOTS-12348 | `*` |
| ⢐ | `braille_2890` | U+2890 | BRAILLE PATTERN DOTS-58 | `*` |
| ⢑ | `braille_2891` | U+2891 | BRAILLE PATTERN DOTS-158 | `*` |
| ⢒ | `braille_2892` | U+2892 | BRAILLE PATTERN DOTS-258 | `*` |
| ⢓ | `braille_2893` | U+2893 | BRAILLE PATTERN DOTS-1258 | `*` |
| ⢔ | `braille_2894` | U+2894 | BRAILLE PATTERN DOTS-358 | `*` |
| ⢕ | `braille_2895` | U+2895 | BRAILLE PATTERN DOTS-1358 | `*` |
| ⢖ | `braille_2896` | U+2896 | BRAILLE PATTERN DOTS-2358 | `*` |
| ⢗ | `braille_2897` | U+2897 | BRAILLE PATTERN DOTS-12358 | `*` |
| ⢘ | `braille_2898` | U+2898 | BRAILLE PATTERN DOTS-458 | `*` |
| ⢙ | `braille_2899` | U+2899 | BRAILLE PATTERN DOTS-1458 | `*` |
| ⢚ | `braille_289A` | U+289A | BRAILLE PATTERN DOTS-2458 | `*` |
| ⢛ | `braille_289B` | U+289B | BRAILLE PATTERN DOTS-12458 | `*` |
| ⢜ | `braille_289C` | U+289C | BRAILLE PATTERN DOTS-3458 | `*` |
| ⢝ | `braille_289D` | U+289D | BRAILLE PATTERN DOTS-13458 | `*` |
| ⢞ | `braille_289E` | U+289E | BRAILLE PATTERN DOTS-23458 | `*` |
| ⢟ | `braille_289F` | U+289F | BRAILLE PATTERN DOTS-123458 | `*` |
| ⢠ | `braille_28A0` | U+28A0 | BRAILLE PATTERN DOTS-68 | `*` |
| ⢡ | `braille_28A1` | U+28A1 | BRAILLE PATTERN DOTS-168 | `*` |
| ⢢ | `braille_28A2` | U+28A2 | BRAILLE PATTERN DOTS-268 | `*` |
| ⢣ | `braille_28A3` | U+28A3 | BRAILLE PATTERN DOTS-1268 | `*` |
| ⢤ | `braille_28A4` | U+28A4 | BRAILLE PATTERN DOTS-368 | `*` |
| ⢥ | `braille_28A5` | U+28A5 | BRAILLE PATTERN DOTS-1368 | `*` |
| ⢦ | `braille_28A6` | U+28A6 | BRAILLE PATTERN DOTS-2368 | `*` |
| ⢧ | `braille_28A7` | U+28A7 | BRAILLE PATTERN DOTS-12368 | `*` |
| ⢨ | `braille_28A8` | U+28A8 | BRAILLE PATTERN DOTS-468 | `*` |
| ⢩ | `braille_28A9` | U+28A9 | BRAILLE PATTERN DOTS-1468 | `*` |
| ⢪ | `braille_28AA` | U+28AA | BRAILLE PATTERN DOTS-2468 | `*` |
| ⢫ | `braille_28AB` | U+28AB | BRAILLE PATTERN DOTS-12468 | `*` |
| ⢬ | `braille_28AC` | U+28AC | BRAILLE PATTERN DOTS-3468 | `*` |
| ⢭ | `braille_28AD` | U+28AD | BRAILLE PATTERN DOTS-13468 | `*` |
| ⢮ | `braille_28AE` | U+28AE | BRAILLE PATTERN DOTS-23468 | `*` |
| ⢯ | `braille_28AF` | U+28AF | BRAILLE PATTERN DOTS-123468 | `*` |
| ⢰ | `braille_28B0` | U+28B0 | BRAILLE PATTERN DOTS-568 | `*` |
| ⢱ | `braille_28B1` | U+28B1 | BRAILLE PATTERN DOTS-1568 | `*` |
| ⢲ | `braille_28B2` | U+28B2 | BRAILLE PATTERN DOTS-2568 | `*` |
| ⢳ | `braille_28B3` | U+28B3 | BRAILLE PATTERN DOTS-12568 | `*` |
| ⢴ | `braille_28B4` | U+28B4 | BRAILLE PATTERN DOTS-3568 | `*` |
| ⢵ | `braille_28B5` | U+28B5 | BRAILLE PATTERN DOTS-13568 | `*` |
| ⢶ | `braille_28B6` | U+28B6 | BRAILLE PATTERN DOTS-23568 | `*` |
| ⢷ | `braille_28B7` | U+28B7 | BRAILLE PATTERN DOTS-123568 | `*` |
| ⢸ | `braille_28B8` | U+28B8 | BRAILLE PATTERN DOTS-4568 | `*` |
| ⢹ | `braille_28B9` | U+28B9 | BRAILLE PATTERN DOTS-14568 | `*` |
| ⢺ | `braille_28BA` | U+28BA | BRAILLE PATTERN DOTS-24568 | `*` |
| ⢻ | `braille_28BB` | U+28BB | BRAILLE PATTERN DOTS-124568 | `*` |
| ⢼ | `braille_28BC` | U+28BC | BRAILLE PATTERN DOTS-34568 | `*` |
| ⢽ | `braille_28BD` | U+28BD | BRAILLE PATTERN DOTS-134568 | `*` |
| ⢾ | `braille_28BE` | U+28BE | BRAILLE PATTERN DOTS-234568 | `*` |
| ⢿ | `braille_28BF` | U+28BF | BRAILLE PATTERN DOTS-1234568 | `*` |
| ⣀ | `braille_28C0` | U+28C0 | BRAILLE PATTERN DOTS-78 | `*` |
| ⣁ | `braille_28C1` | U+28C1 | BRAILLE PATTERN DOTS-178 | `*` |
| ⣂ | `braille_28C2` | U+28C2 | BRAILLE PATTERN DOTS-278 | `*` |
| ⣃ | `braille_28C3` | U+28C3 | BRAILLE PATTERN DOTS-1278 | `*` |
| ⣄ | `braille_28C4` | U+28C4 | BRAILLE PATTERN DOTS-378 | `*` |
| ⣅ | `braille_28C5` | U+28C5 | BRAILLE PATTERN DOTS-1378 | `*` |
| ⣆ | `braille_28C6` | U+28C6 | BRAILLE PATTERN DOTS-2378 | `*` |
| ⣇ | `braille_28C7` | U+28C7 | BRAILLE PATTERN DOTS-12378 | `*` |
| ⣈ | `braille_28C8` | U+28C8 | BRAILLE PATTERN DOTS-478 | `*` |
| ⣉ | `braille_28C9` | U+28C9 | BRAILLE PATTERN DOTS-1478 | `*` |
| ⣊ | `braille_28CA` | U+28CA | BRAILLE PATTERN DOTS-2478 | `*` |
| ⣋ | `braille_28CB` | U+28CB | BRAILLE PATTERN DOTS-12478 | `*` |
| ⣌ | `braille_28CC` | U+28CC | BRAILLE PATTERN DOTS-3478 | `*` |
| ⣍ | `braille_28CD` | U+28CD | BRAILLE PATTERN DOTS-13478 | `*` |
| ⣎ | `braille_28CE` | U+28CE | BRAILLE PATTERN DOTS-23478 | `*` |
| ⣏ | `braille_28CF` | U+28CF | BRAILLE PATTERN DOTS-123478 | `*` |
| ⣐ | `braille_28D0` | U+28D0 | BRAILLE PATTERN DOTS-578 | `*` |
| ⣑ | `braille_28D1` | U+28D1 | BRAILLE PATTERN DOTS-1578 | `*` |
| ⣒ | `braille_28D2` | U+28D2 | BRAILLE PATTERN DOTS-2578 | `*` |
| ⣓ | `braille_28D3` | U+28D3 | BRAILLE PATTERN DOTS-12578 | `*` |
| ⣔ | `braille_28D4` | U+28D4 | BRAILLE PATTERN DOTS-3578 | `*` |
| ⣕ | `braille_28D5` | U+28D5 | BRAILLE PATTERN DOTS-13578 | `*` |
| ⣖ | `braille_28D6` | U+28D6 | BRAILLE PATTERN DOTS-23578 | `*` |
| ⣗ | `braille_28D7` | U+28D7 | BRAILLE PATTERN DOTS-123578 | `*` |
| ⣘ | `braille_28D8` | U+28D8 | BRAILLE PATTERN DOTS-4578 | `*` |
| ⣙ | `braille_28D9` | U+28D9 | BRAILLE PATTERN DOTS-14578 | `*` |
| ⣚ | `braille_28DA` | U+28DA | BRAILLE PATTERN DOTS-24578 | `*` |
| ⣛ | `braille_28DB` | U+28DB | BRAILLE PATTERN DOTS-124578 | `*` |
| ⣜ | `braille_28DC` | U+28DC | BRAILLE PATTERN DOTS-34578 | `*` |
| ⣝ | `braille_28DD` | U+28DD | BRAILLE PATTERN DOTS-134578 | `*` |
| ⣞ | `braille_28DE` | U+28DE | BRAILLE PATTERN DOTS-234578 | `*` |
| ⣟ | `braille_28DF` | U+28DF | BRAILLE PATTERN DOTS-1234578 | `*` |
| ⣠ | `braille_28E0` | U+28E0 | BRAILLE PATTERN DOTS-678 | `*` |
| ⣡ | `braille_28E1` | U+28E1 | BRAILLE PATTERN DOTS-1678 | `*` |
| ⣢ | `braille_28E2` | U+28E2 | BRAILLE PATTERN DOTS-2678 | `*` |
| ⣣ | `braille_28E3` | U+28E3 | BRAILLE PATTERN DOTS-12678 | `*` |
| ⣤ | `braille_28E4` | U+28E4 | BRAILLE PATTERN DOTS-3678 | `*` |
| ⣥ | `braille_28E5` | U+28E5 | BRAILLE PATTERN DOTS-13678 | `*` |
| ⣦ | `braille_28E6` | U+28E6 | BRAILLE PATTERN DOTS-23678 | `*` |
| ⣧ | `braille_28E7` | U+28E7 | BRAILLE PATTERN DOTS-123678 | `*` |
| ⣨ | `braille_28E8` | U+28E8 | BRAILLE PATTERN DOTS-4678 | `*` |
| ⣩ | `braille_28E9` | U+28E9 | BRAILLE PATTERN DOTS-14678 | `*` |
| ⣪ | `braille_28EA` | U+28EA | BRAILLE PATTERN DOTS-24678 | `*` |
| ⣫ | `braille_28EB` | U+28EB | BRAILLE PATTERN DOTS-124678 | `*` |
| ⣬ | `braille_28EC` | U+28EC | BRAILLE PATTERN DOTS-34678 | `*` |
| ⣭ | `braille_28ED` | U+28ED | BRAILLE PATTERN DOTS-134678 | `*` |
| ⣮ | `braille_28EE` | U+28EE | BRAILLE PATTERN DOTS-234678 | `*` |
| ⣯ | `braille_28EF` | U+28EF | BRAILLE PATTERN DOTS-1234678 | `*` |
| ⣰ | `braille_28F0` | U+28F0 | BRAILLE PATTERN DOTS-5678 | `*` |
| ⣱ | `braille_28F1` | U+28F1 | BRAILLE PATTERN DOTS-15678 | `*` |
| ⣲ | `braille_28F2` | U+28F2 | BRAILLE PATTERN DOTS-25678 | `*` |
| ⣳ | `braille_28F3` | U+28F3 | BRAILLE PATTERN DOTS-125678 | `*` |
| ⣴ | `braille_28F4` | U+28F4 | BRAILLE PATTERN DOTS-35678 | `*` |
| ⣵ | `braille_28F5` | U+28F5 | BRAILLE PATTERN DOTS-135678 | `*` |
| ⣶ | `braille_28F6` | U+28F6 | BRAILLE PATTERN DOTS-235678 | `*` |
| ⣷ | `braille_28F7` | U+28F7 | BRAILLE PATTERN DOTS-1235678 | `*` |
| ⣸ | `braille_28F8` | U+28F8 | BRAILLE PATTERN DOTS-45678 | `*` |
| ⣹ | `braille_28F9` | U+28F9 | BRAILLE PATTERN DOTS-145678 | `*` |
| ⣺ | `braille_28FA` | U+28FA | BRAILLE PATTERN DOTS-245678 | `*` |
| ⣻ | `braille_28FB` | U+28FB | BRAILLE PATTERN DOTS-1245678 | `*` |
| ⣼ | `braille_28FC` | U+28FC | BRAILLE PATTERN DOTS-345678 | `*` |
| ⣽ | `braille_28FD` | U+28FD | BRAILLE PATTERN DOTS-1345678 | `*` |
| ⣾ | `braille_28FE` | U+28FE | BRAILLE PATTERN DOTS-2345678 | `*` |
| ⣿ | `braille_28FF` | U+28FF | BRAILLE PATTERN DOTS-12345678 | `*` |
