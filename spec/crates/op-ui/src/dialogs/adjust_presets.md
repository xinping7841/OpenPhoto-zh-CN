# dialogs/adjust_presets.rs: Photoshop's adjustment presets

## Simplified Chinese interface fork

This Chinese-only fork renders the component's built-in user-facing dialog copy in Simplified Chinese using direct UI literals. This requirement overrides English wording parity with Photoshop, including the English copy examples below; those examples identify controls and retain the measured interaction and layout context, not a requirement to display English. Titles, labels, buttons, option names, built-in preset display names, tooltips and confirmation copy are localized where this component owns them. Keyboard shortcuts and technical names such as RGB, JPEG and ICC profile identifiers remain unchanged.

Identifiers, enum variants and match keys, file extensions, preset folder paths, binary/text format keys, algorithm tables and numerical defaults are unchanged. Document titles, layer names, user-saved preset names and other supplied user data are displayed verbatim, not translated. In-scope tests assert the translated built-in display copy while preserving protocol IDs and data fixtures. Shared or host-supplied copy outside this directory is governed by its owning component; localization here does not rewrite it. The measured geometry remains unchanged, so Chinese font coverage and clipping require visual verification.

## Responsibility

The presets Photoshop 2026 ships for its adjustment dialogs, as the numbers its preset files hold (read from `Presets/Levels/*.alv`, `Curves/*.acv`, `Hue and Saturation/*.ahu`, `Black and White/*.blw`, `Channel Mixer/*.cha`, `Exposure/*.eap` in Photoshop's application folder and written out as constants, sorted by name as Photoshop's menus list them). Only numbers are kept; no Adobe file is bundled.

## Tables

- `LEVELS`: per channel (RGB, Red, Green, Blue) input black, gamma × 100, input white, output black, output white (the `.alv` records after the version word).
- `CURVES`: per channel the (input, output) points (`.acv` stores output first).
- `HUE_SATURATION`: Master hue, saturation, lightness, and the Colorize values for the colorizing ones (Cyanotype, Sepia; the hue stored as −145 is 215°). Every preset leaves the six ranges at their defaults.
- `BLACK_WHITE`: the six weights (Reds, Yellows, Greens, Cyans, Blues, Magentas) from the `.blw` descriptor; no preset uses a tint.
- `CHANNEL_MIXER`: all six are monochrome; the gray output's Red, Green, Blue.
- `EXPOSURE`: exposure, offset, gamma (three floats after the version word).

## Used by

`levels.md`, `curves.md`, `hue_saturation.md`, `black_white.md`, `channel_mixer.md`, `exposure.md` (their Preset menus and the preset name shown).

## Tests

Each dialog's `photoshops_presets` test picks a preset, checks the values and that the Preset shows its name, and that a change makes it Custom.
