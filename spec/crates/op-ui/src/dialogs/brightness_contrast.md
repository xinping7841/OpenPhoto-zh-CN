# dialogs/brightness_contrast.rs: Brightness/Contrast dialog

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component responsibility

The dialog for Image › Adjustments › Brightness/Contrast..., rebuilt pixel by pixel after Photoshop 2026's UXP dialog (401 × 212 pt, within 1 pixel of Photoshop screenshots). It only manages the settings; preview and apply are handled by the host in `adjust.rs` and by `lib.rs`.

## Data and defaults

- `brightness` (−150–150) and `contrast` (−50–100) are field text, defaulting to 0; `legacy` (Use Legacy) is unchecked by default. Defaults are restored every time the dialog opens.
- `adjustment()`: returns `Adjustment::BrightnessContrast` when both values are valid, otherwise `None` (OK grayed out, no preview). Decimals are rounded.

## Layout (Photoshop points, from the dialog's top-left corner)

- "Brightness" label at x 20, field (203, 48)–(260, 72), slider track at y 84, x 20–260.
- "Contrast" label, field (203, 103)–(260, 127), slider at y 139. The slider's 0 is at the midpoint of the track (Contrast's negative half only goes to −50). The track is `#737373`.
- "Use Legacy" checkbox (20, 164); "Preview (Opt+P)" checkbox (272, 174).
- Right-side button column: OK, Cancel, Auto (`uxp::buttons`).
- On open, the Brightness field gets focus with everything selected.

## Interaction

- The fields accept typing, with ↑/↓ stepping by 1 (Shift 10); dragging or clicking a slider changes the value.
- Auto: picks brightness and contrast from the layer histogram. Photoshop's Auto rule could not be reproduced (for example, for an even 0–255 gray ramp it gives 14 / −19, and for 128–255 it gives 0 / 0), so here the pair on Photoshop's brightness/contrast curves that comes closest to the Auto Contrast stretch (clipping 0.1% at each end) is chosen (histogram-weighted squared error, step 2).
- Enter / OK applies, recording the history "Brightness/Contrast"; Esc / Cancel cancels; ⌥P toggles preview.

## Test coverage

- `ui_tests::brightness_contrast_dialog_types_and_applies`: after typing 50 and checking Use Legacy, 100 gray becomes 150; when unchecked, the result equals Photoshop's brightness curve.
- Screenshot comparison: `ui_tests::screenshot_adjustment_dialogs` (`brightness_contrast_dialog.png`).
