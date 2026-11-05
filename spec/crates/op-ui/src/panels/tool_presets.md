# panels/tool_presets.rs: Tool Presets panel

## Responsibilities

The Tool Presets panel (Window › Tool Presets) is a floating panel (`floating.md`) listing the tool presets (`tool_presets.md`).

## Layout and interactions

- **List:** the presets in a scroll area, each a row with its name and a 🗑 button.
  - With "Current Tool Only" (`AppState::tool_presets_current_only`, on by default), only the current tool's presets are listed.
  - "No tool presets." shows when there are none.
  - Clicking a name applies the preset (`tool_presets::apply`). The 🗑 button deletes it.
- **Below the list:** the "Current Tool Only" checkbox and "New Tool Preset...".
- **New Tool Preset...** (`open_new`, also in the options bar's picker) opens the name dialog (`dialogs/new_preset.md`, titled "New Tool Preset", suggesting `suggested_name`). `lib.rs` saves the preset on OK.

## Known limitations

- The panel uses egui's widgets and has no tool icons. There is no panel menu (sort by tool, rename, reset, load or save sets).

## Test coverage

`ui_tests::tool_presets_save_and_pick`:

- New Tool Preset... from the options bar's picker saves "Brush Tool 1";
- from the Eraser, with Current Tool Only off, the panel picks it, bringing back the Brush at 77 px;
- screenshot `tool_presets_panel.png`.


## Simplified Chinese fork wording

This Chinese-only fork uses direct Simplified Chinese UI literals. This wording overrides the upstream English copy and English-parity descriptions above; those English names identify the same controls and behavior, not a requirement to render English. Specifications remain in English.

Empty state, current-tool filter, create action, delete tooltip and name-dialog title use Simplified Chinese. Preset names remain verbatim user data; creation, filtering, application and deletion keep their existing behavior.

The localization does not change shortcuts, algorithm keys, user-entered data or icon artwork. Layout dimensions remain upstream-derived; Chinese font coverage and clipping require visual verification in the application. Tests outside this panel subtree are not updated by this scoped change.
