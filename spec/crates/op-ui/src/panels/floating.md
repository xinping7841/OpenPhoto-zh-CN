# panels/floating.rs: floating panels

## Component responsibilities

Panels opened from the Window menu that are not docked in the right-hand panel column: Info, Navigator, Histogram, Clone Source (`clone_source.md`), Brushes and Brush Settings (`brushes.md`), Tool Presets (`tool_presets.md`), Layer Comps (`layer_comps.md`). Each panel has its own frame and can be dragged.

## State

`AppState::floating` (`FloatingPanels`): whether each of the eight panels is open. `toggle` toggles it. Positions are remembered by egui's Area (kept after dragging).

## Panel menus

The Info and Navigator headers have a panel menu icon (four lines, 22 pt left of the close button) whose menu offers Panel Options... (`panel_options.md`).

## Layout and visuals

- Frame: panel background color, 1 pt stroke in the divider color, shadow.
- Header 26 pt: tab bar background color, a 90 pt wide tab on the left (panel background color, panel name in semibold), and a close button on the right (×, brighter on hover) that closes the panel when clicked.
- The content area has 10 pt of margin on all sides. Sizes (excluding the header): Navigator 250 × 230, Histogram 250 × 150 (260 tall in its expanded view), Clone Source 250 × 250, Brushes 250 × 260, Brush Settings 300 × 240, Tool Presets 250 × 200, Layer Comps 280 × 200; Info is Photoshop's 214 pt wide and as tall as its sections (`info::height`), drawing over the margin.
- Default position: to the left of the icon column, arranged in a column from top to bottom in the order Navigator, Histogram, Info, Clone Source, Brushes, Brush Settings, Tool Presets, Layer Comps, with 8 pt gaps; each panel's default position is fixed (regardless of whether other panels are open), so a panel opened later does not cover one opened earlier.

## Known limitations

- Panels cannot be docked into the panel column, collapsed to icons, or grouped with other panels into tab groups; there is no panel menu.
- The open state is not remembered after the application closes.


## Simplified Chinese fork wording

This Chinese-only fork uses direct Simplified Chinese UI literals. This wording overrides the upstream English copy and English-parity descriptions above; those English names identify the same controls and behavior, not a requirement to render English. Specifications remain in English.

Panel titles, Panel Options entry, option-dialog titles, color label and confirmation buttons use Simplified Chinese. Floating panel IDs continue to use their original English salts, independently of the displayed title.

The localization does not change shortcuts, algorithm keys, user-entered data or icon artwork. Layout dimensions remain upstream-derived; Chinese font coverage and clipping require visual verification in the application. Tests outside this panel subtree are not updated by this scoped change.
