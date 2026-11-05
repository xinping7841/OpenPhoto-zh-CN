# dialogs/document_presets.rs: New Document presets

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component responsibility

The data and drawing used by the New Document dialog (`new_document.md`): length units, presets, the preset list for each category, and the icons on the cards.

## Units (`Unit`)

Pixels, Inches, Centimeters, Millimeters, Points, Picas. `pixels(ppi)` gives how many pixels one unit equals (1 inch = 2.54 centimeters = 25.4 millimeters = 72 points = 6 picas); `short` is the abbreviation on the card (px, in, cm, mm, pt, pica); `format` displays a value: pixels are rounded to integers, others have at most 3 decimals with trailing zeros removed.

## Presets (`Preset`)

Name, width, height, unit, resolution (ppi), and the kind of card icon (`Kind`). `size_label` is the size line on the card, e.g. "7 x 5 in @ 300 ppi"; `pixels` converts to pixels.

`category(index)` gives each page's blank document presets in Photoshop 2026's order:

- **Photo** (9, inches, 300 ppi): Default Photoshop Size 7 × 5, Landscape 3 × 2, 6 × 4, 7 × 5, 10 × 8, Portrait 2 × 3, 4 × 6, 5 × 7, 8 × 10.
- **Print** (300 ppi): Letter, Legal, Tabloid (inches); A4, A6, A5, A3, B5, B4, B3, C4, C5 (millimeters).
- **Art & Illustration** (300 ppi): 1000 / 2000 pixel grid, Poster 18 × 24 in, Postcard 4 × 6 in, 1080p, 720p.
- **Web** (pixels, 72 ppi): Web Most Common 1366 × 768, Web Large, Web Medium, Web Minimum, Web Small, MacBook Pro 13 / 15 (Retina), iMac 27, Desktop HD Design.
- **Mobile** (pixels, 72 ppi): iPhone, iPad, Android, Surface, Apple Watch, Mobile Design, iOS 7 and Mac icon sizes, 24 in total.
- **Film & Video** (pixels, 72 ppi): HDTV, HDV, DVCPRO HD, DCI 2K/4K/8K, UHDTV, FUHDTV, NTSC, PAL, Cineon, Film (2K)/(4K), 24 in total.

## Keeping Recent and Saved (`store_path`, `serialize`, `parse`)

- The file is `platform::data_dir()/new-document-presets.txt` (`store_path`; see `../platform.md` for Windows, macOS and Linux roots). The filename and serialized field names remain unchanged, and root resolution does not migrate or delete existing data.
- `serialize` writes one line per preset, Recent first (newest first), then Saved. Each line has these fields, separated by tabs:
  1. `recent` or `saved`
  2. the name (tabs and line breaks become spaces)
  3. the width
  4. the height
  5. the unit, as an index into `Unit::ALL`
  6. the ppi
  7. the card's kind: `custom`, `photo`, `page`, `grid`, `brush`, `browser`, `phone`, `tablet`, `watch`, `app-icon`, `surface`, `strip`, `video`, or `video:<badge>`
- `parse` reads the file back and skips lines that don't read. A video badge is looked up among Film & Video's; an unknown badge reads as a video without one.

## Card icons (`paint_icon`)

1 pt `#b9b9b9` lines; a frame is drawn with the preset's aspect ratio: the longest side runs from 36 pt to 82 pt according to the square root of its size within the page (`scales`: the position of the square root of the pixel area within the page, 0–1), and the other side is at most 62 pt (measured from the cards on Photoshop's Photo page). Depending on the kind, the frame contains: photo (mountains and sun), page (folded corner; Custom draws a crosshair outside the top-left corner), dot grid, brush, browser (top bar), phone/tablet (rounded corners and Home button), Surface (stylus on the right), watch (watch band), app icon (rounded dot grid), thin strip, video (play triangle and 2K/4K/8K corner badge).

## Known limitations

- The icons are approximate drawings made from shapes, not Photoshop's icon assets; Print, Mobile, and Film & Video list only the presets visible in the screenshots (Photoshop has 14, 28, and 25 respectively).

## Test coverage

- `recent_and_saved_round_trip`: a video preset with a badge and a print preset in Recent and a custom one in Saved read back the same (a tab in a name becomes a space); a short line is skipped.
- `presets_convert_to_pixels`: Photo has 9 and the first is 2100 × 1500 pixels; A4 is 2480 × 3508; the Web size line; three decimals for inches.
