# dialogs/threshold.rs: Threshold dialog

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component responsibilities

The dialog for Image › Adjustments › Threshold..., redone after Photoshop 2026's classic dialog (381 × 191 pt).

## Data and layout

- Threshold Level (1–255, default 128). "Threshold Level:" right-aligned to x 171.5, y 47.25; input field (177, 38)–(213.5, 57) (`appkit::field`), which takes focus and selects all when opened.
- Luminosity histogram (15, 66)–(270, 166), `#454545` background, `#d0d0d0` vertical bars, normalized by the maximum count (the luminosity histogram within the active layer's selection at the time of opening).
- Below it a single white slider (tip at y 167.5), with 0 and 255 at x 14.25 and 268.75; pressing or dragging in the slider's row sets the level.
- Buttons at x 290–370.5: OK (38.5), Cancel (73.5); "Preview" checkbox (290, 118).

## Test coverage

- `level_range`; `ui_tests::adjustments_with_dialogs`; screenshot `threshold_dialog.png`.
