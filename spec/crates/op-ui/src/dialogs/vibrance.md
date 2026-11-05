# dialogs/vibrance.rs: Vibrance and Posterize dialogs

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component responsibilities

The dialogs for Image › Adjustments › Vibrance... and Posterize..., rebuilt after the Photoshop 2026 UXP dialogs (401 × 166 and 401 × 164 pt).

## Vibrance

- Vibrance, Saturation (−100–100, default 0). Labels at (20, 60.5) / (20, 115.5), input boxes at x 215–260, y 48 / 103, tracks at y 84 / 139 (x 20–260, `#737373`, 0 centered). On open, the Vibrance box takes focus with its contents selected.
- "Preview (Opt+P)" at (272, 127); OK and Cancel on the right.

## Posterize

- Levels (2–255, default 4): label at (20, 60.5), input box (220, 48)–(260, 72), track at y 84, mapping linearly from 2 at the left end to 255 at the right end (`uxp::linear_slider`). On open, the input box takes focus with its contents selected.
- "Preview (Opt+P)" at (272, 127).

## Test coverage

- `defaults_and_ranges`; `ui_tests::small_uxp_adjustment_dialogs_apply`; screenshots `vibrance_dialog.png`, `posterize_dialog.png`.
