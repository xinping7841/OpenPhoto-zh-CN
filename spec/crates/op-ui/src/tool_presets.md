# tool_presets.rs: tool presets

## Responsibilities

A tool preset is a tool with its options saved under a name. Presets are made with New Tool Preset... and picked from the Tool Presets panel (`panels/tool_presets.md`) or the options bar's Tool Presets picker (`options_bar.md`).

## Data

- **`ToolPreset`** holds:
  - the name and the tool;
  - the tool's `PaintOptions` (painting tools), or None;
  - the values of the options bar settings the tool keeps under keys (`options_tools::setting_keys`: the keys of its toggles, pop-ups, fields, percentages, checkboxes and other controls that set values), as far as they have been set.
- **`AppState::tool_presets`** keeps the presets for the session.

## Functions

- **`capture(app, name)`:** a preset of the current tool as its options are now.
- **`apply(app, preset)`:** selects the preset's tool and puts back its brush and settings.
- **`suggested_name(app, tool)`:** the tool's name and the next number among its presets ("Brush Tool 1"), as Photoshop suggests it.

## Known limitations

- Presets last for the session only; Photoshop keeps them in its preset files.
- Photoshop's "Include Color" (the foreground color) is not offered.
- Settings that live outside the keyed options (marquee, crop, type and shape options, gradients) are not saved.

## Test coverage

- `capture_and_apply`: a 77 px Multiply brush saved, then restored from the Eraser.
- `ui_tests::tool_presets_save_and_pick`: see `panels/tool_presets.md`.


## Simplified Chinese fork wording

This Chinese-only fork uses direct Simplified Chinese UI literals. Chinese display wording overrides upstream English-copy parity described above; the English names in this specification identify controls and behavior, rather than requiring English rendering. Specifications remain in English.

New preset suggestions use the localized tool display name followed by the existing sequence number. Capturing and applying preserve user-supplied names, tool enums, keyed settings and paint options verbatim. The scoped capture/apply test expects the Chinese default brush suggestion and verifies that an English user name stays unchanged.

Chinese glyph coverage and clipping in upstream-derived fixed control dimensions require visual verification. This change does not alter algorithm keys, identifiers, user data, shortcuts or icon artwork.
