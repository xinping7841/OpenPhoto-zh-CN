# panels/navigator.rs: Navigator panel

## Component responsibilities

A thumbnail of the document and the current view position, plus zoom controls.

## Content and interaction

- Thumbnail: the composite image scaled proportionally to fit the panel (`DocState::composite_texture`, cached by revision, longest side 256 pixels), with a checkerboard where transparent.
- View box: the part of the document visible in the window (`document_view::visible_rect`), clipped to the thumbnail, a 1 pt line in Navigator Panel Options' color (`AppState::navigator_box`: Light Red, Photoshop's default, Cyan, Green, Light Blue, Yellow, Magenta; `panel_options::VIEW_BOX_COLORS`).
- Clicking or dragging in the thumbnail area: moves the document position under the pointer to the window center (`document_view::center_on`).
- Bottom: the current zoom percentage in a field (typing a number, with or without %, and pressing Enter zooms there about the window center, 1%–12800%); a zoom out button (small mountains: one zoom step down, `zoom_step`), a logarithmic zoom slider (1%–12800%) and a zoom in button (large mountains: a step up). Dragging the slider zooms about the window center (`zoom_to`).

## Known limitations

- Navigator Panel Options offers named colors only (not Custom...).


## Simplified Chinese fork wording

This Chinese-only fork uses direct Simplified Chinese UI literals. This wording overrides the upstream English copy and English-parity descriptions above; those English names identify the same controls and behavior, not a requirement to render English. Specifications remain in English.

Zoom-in and zoom-out tooltips use Simplified Chinese. Percentage formatting, zoom values, icon artwork and interaction IDs remain unchanged.

The localization does not change shortcuts, algorithm keys, user-entered data or icon artwork. Layout dimensions remain upstream-derived; Chinese font coverage and clipping require visual verification in the application. Tests outside this panel subtree are not updated by this scoped change.
