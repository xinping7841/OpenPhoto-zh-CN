# dialogs/new_preset.rs: naming a new preset

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component Responsibilities

The small dialog asking for a preset's name, used by the Crop tool's New Crop Preset... (`crop_tool.md`); `lib.rs` shows it while `AppState::new_crop_preset` is set and saves the preset on OK.

## Layout (measured in Photoshop 2026's New Crop Preset dialog, 448 × 110 pt)

- Centered on the window, with the shared frame and title (`common::frame`, 13 pt bold) given by its user: "New Crop Preset" (Crop tool), "Pattern Name" (Edit › Define Pattern..., `commands.md`; Photoshop's also shows the pattern's thumbnail, which this one doesn't).
- "Name:" (AppKit 12 pt text) right-aligned to x 55.5 at y 47.5; the field (60, 38)–(362.5, 57), `#454545` with a `#5e5e5e` border, focused with the suggested name selected on opening.
- OK (377.5, 38.5)–(438.5, 64.5), the default button (grayed out while the name is empty); Cancel (377.5, 73.5)–(438.5, 99.5).

## Interaction

OK or Enter returns `Outcome::Ok(name)` (trimmed); Cancel or Escape `Outcome::Cancel`. Modal while open (`AppState::modal_open`).

## Test Coverage

`ui_tests::crop_presets_front_image_and_shield_color` (see `crop_tool.md`); its `new_crop_preset` screenshot was compared with Photoshop's dialog.
