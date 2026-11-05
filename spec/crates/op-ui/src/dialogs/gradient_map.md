# dialogs/gradient_map.rs: Gradient Map Dialog

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Component Responsibilities

The dialog for Image › Adjustments › Gradient Map..., rebuilt as Photoshop 2026's classic dialog (416 × 230 pt). The algorithm is in `op-core`'s `adjust.md` and `gradient.md`.

## Data and Layout

- The gradient (`op_core::gradient::Gradient`) is Foreground to Background on open (`AdjustDialog::set_gradient_colors`); Dither and Reverse are unchecked by default; Method defaults to Smooth (Perceptual / Linear / Classic / Smooth).
- Group box "Gradient Used for Grayscale Mapping" (11, 46.5)–(309, 97): gradient box (20, 67.5)–(300, 87.5); the gradient bar (21, 68.5)–(286, 86.5) is drawn (over a checkerboard where see-through) according to the current method and Reverse. Clicking the bar sets `wants_editor`; the app (`AdjustDialog::take_editor_request`) opens the Gradient Editor (`gradient_editor.md`) on the dialog's gradient, and the editor's OK replaces it (`set_map_gradient`) while Cancel leaves it. The ⌄ on the right opens the editor's presets as a native pop-up menu (Foreground to Background and Foreground to Transparent take the dialog's first and last stop colors), the current one checked.
- Group box "Gradient Options" (11, 121)–(309, 219): "Dither" (20, 139) and "Reverse" (20, 166) checkboxes, "Method:" (20.5, 200) and the pop-up menu (72, 189.5)–(157, 210.5).
- Buttons x 325.5–385.5: OK (45), Cancel (80); "Preview" (325, 124.5).

## Mapping

- Mapping: two opaque end stops with a 50% midpoint and Smoothness 100%, without Dither, map through `Adjustment::GradientMap` (measured against Photoshop); anything else, or Dither, maps through `Adjustment::GradientTable` (the gradient's 256-entry table, interpolated at the exact luminosity, Dither adding up to ±half a level of repeatable noise). Opacity stops are ignored by the mapping, as in Photoshop.

## Known Limitations

- Multi-stop and dithered mapping have not been compared level by level with Photoshop.

## Test Coverage

- `reverse_method_and_stops` (Dither or a third stop switches to the table); `ui_tests::gradient_editor_for_the_tool_and_gradient_map` (the bar opens the editor, Escape cancels it); `ui_tests::more_adjustments` (Smooth mapping matches the core); screenshot `gradient_map_dialog.png`.
