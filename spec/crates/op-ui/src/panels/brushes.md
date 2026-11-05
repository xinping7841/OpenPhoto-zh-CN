# panels/brushes.rs: Brushes and Brush Settings panels

## Responsibilities

Two floating panels (`floating.md`) for the current painting tool's brush (`AppState::paint_options`):

- **Brushes** (Window › Brushes) shows the presets.
- **Brush Settings** (Window › Brush Settings, F5, or the options bar's Brush Settings toggle) shows the tip and what the pen's pressure controls.

Without a painting tool, each panel says so.

## Brushes (`brushes`)

- **Size:** 1–5000 px, on a logarithmic slider.
- **"General Brushes":** the presets (`brush_presets::GENERAL`), with the current one highlighted. Clicking one applies it (`brush_presets::apply`).

## Brush Settings (`brush_settings`)

- **Brush Tip Shape:**
  - Size (as above);
  - Angle (−180–180°) and Roundness (1–100%);
  - Hardness (0–100%);
  - Spacing (1–1000% of the diameter, logarithmic).
- **Pen pressure:** three switches that set `PaintOptions::pressure`:
  - "Shape Dynamics: Size Jitter — Pen Pressure" (size);
  - "Transfer: Opacity Jitter — Pen Pressure" (opacity);
  - "Transfer: Flow Jitter — Pen Pressure" (flow).

## Known limitations

- Both panels use egui's widgets and are not rebuilds of Photoshop's.
- Brush Settings has no section list. It also has no jitter amounts, minimums, other controls (fade, tilt, stylus wheel), Scattering, Texture, Dual Brush, Color Dynamics, Brush Pose, Noise, Wet Edges, Build-up, Smoothing (that is on the options bar) or Protect Texture, and no tip preview.
- Brushes has no folders other than General Brushes, no search and no thumbnails.

## Test coverage

`ui_tests::brushes_and_brush_settings_panels`:

- the options bar's toggle opens Brush Settings;
- its size pressure switch reaches the brush;
- Window › Brushes' Hard Round sets the hardness and preset, and resets the dynamics;
- screenshot `brush_settings_panel.png`.


## Simplified Chinese fork wording

This Chinese-only fork uses direct Simplified Chinese UI literals. This wording overrides the upstream English copy and English-parity descriptions above; those English names identify the same controls and behavior, not a requirement to render English. Specifications remain in English.

The empty-tool messages, size, angle, roundness, hardness, spacing, General Brushes, tip-shape heading and pen-pressure controls use Simplified Chinese. Known built-in brush names are translated for display only; the preset index and stored name are unchanged.

The localization does not change shortcuts, algorithm keys, user-entered data or icon artwork. Layout dimensions remain upstream-derived; Chinese font coverage and clipping require visual verification in the application. Tests outside this panel subtree are not updated by this scoped change.
