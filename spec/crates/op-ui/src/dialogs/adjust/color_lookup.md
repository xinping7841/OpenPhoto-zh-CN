# dialogs/adjust/color_lookup.rs: Color Lookup

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Responsibility

Image › Adjustments › Color Lookup..., drawn as Photoshop 2026's UXP dialog as part of `AdjustDialog` (`adjust.md`). Its settings (`Kind::params`): 0 the 3DLUT File pop-up's choice (`Extra::labels`: "Load 3D LUT...", then the cube files), 1 the source radio (3DLUT File, Abstract, Device Link), 2 Dither (on by default).

## Layout

Measured from a 2x capture of Photoshop 2026's dialog: 519 × 186 pt, positions in points from the dialog's top-left corner, controls from the UXP kit (`uxp.md`).

- Title bar: `common::frame`, "Color Lookup".
- Three radios centered at x 25.75 and y 59.75, 88.75, 117.75 (29 pt apart), labels 15.5 pt after the center: 3DLUT File, Abstract, Device Link.
- Their pop-ups x 110–377.5, 24 pt tall from y 48, 77 and 106 (`common::ps_dropdown`: 3 pt corners, the text 10 pt in, the chevron 13.5 pt from the right edge). The 3DLUT File pop-up lists "Load 3D LUT...", a separator, then the cube files; the others show "Load Abstract Profile..." and "Load DeviceLink Profile...".
- Dither checkbox, its box at (20, 148).
- OK (focused, the default) and Cancel down the right (`uxp::buttons`), Preview (Opt+P) with its box at (390, 127.5).

Compared side by side with the capture (`ui_tests::more_filters_and_adjustments_apply`'s `color_lookup.png`), every control lands on Photoshop's.

## Behavior

- Picking a cube file loads it into the lookup registry and selects 3DLUT File; "Load 3D LUT..." opens the open panel and adds the file (`AdjustDialog::pick`).
- The effect is `Adjustment::ColorLookup(id, dither)` while 3DLUT File is chosen and a cube is loaded. Dither adds up to half a level of noise before rounding (`op-core`'s `adjust.md`).
- OK is always on, as in Photoshop: with no cube it just closes the dialog without a change.

## Known limitations

- Abstract and Device Link apply ICC profiles, which need color management (P2 #19): their radios and pop-ups are drawn off and can't be chosen.
