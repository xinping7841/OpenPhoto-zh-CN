# dialogs/exposure.rs: Exposure dialog

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component responsibilities

The dialog for Image › Adjustments › Exposure..., rebuilt after Photoshop 2026's UXP dialog (401 × 257 pt). For the algorithm see `op-core`'s `adjust.md`.

## Data and layout

- Exposure (−20–20), Offset (−0.5–0.5), Gamma Correction (0.01–9.99), defaulting to 0, 0, 1; values are displayed with trailing zeros removed, and Gamma is displayed as "+1" (as Photoshop does).
- "Preset" (20, 61), dropdown (56, 48.5)–(231, 73.5) (Default, Custom, then Photoshop's Minus 1.0, Minus 2.0, Plus 1.0, Plus 2.0 from `adjust_presets::EXPOSURE`; positive values are shown with "+"), preset menu icon (248, 61), whose menu saves, loads and deletes preset files (`preset_files.md`); saved presets are listed at the end of the Preset menu.
- Three rows: labels at y 97 / 152 / 207, input fields at x 201–260, y 84 / 139 / 194, tracks 36 pt below the fields. The Exposure and Offset sliders are centered on 0 (step 0.01, 0.0001); Gamma is linear from 0.01 to 9.99 (step 0.01).
- Three eyedroppers (283.5 / 319.5 / 356, 133): the selected one (the white point when opened) is drawn on a `#d8d8d8` rounded background; the eyedroppers themselves cannot sample from the document.
- "Preview (Opt+P)" (272, 162); OK and Cancel on the right. When opened, the Exposure field gets focus with its contents selected.

## Test coverage

- `defaults_and_numbers`; `ui_tests::small_uxp_adjustment_dialogs_apply`; screenshot `exposure_dialog.png`.
