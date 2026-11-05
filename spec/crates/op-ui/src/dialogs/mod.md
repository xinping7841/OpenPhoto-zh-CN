# dialogs/mod.rs: Modal Dialogs

Only exports the dialogs: Canvas Size (`canvas_size.md`), Fill (`fill.md`), Image Size (`image_size.md`), New (`new_document.md`), New Guide (`new_guide.md`), Select › Modify (`modify_selection.md`), Trim (`trim.md`), the "Save changes?" confirmation (`save_changes.md`), the adjustment and filter dialogs (`adjust.md`; each adjustment dialog has its own module and spec, the classic filter dialog layout is in `filter_layout.md`, and shared controls are in `uxp.md`, `appkit.md`), the Equalize prompt (`equalize.md`), the Custom filter (`custom_filter.md`), the plugin-style distort dialog layout (`distort.md`), and the Color Picker (`color_picker.md`). Shared parts such as the frame and buttons are in `common.md`.

The Chinese-only fork exposes `common` within the UI crate so options-bar rendering can reuse its finite built-in display-label adapter without changing external enum names or serialized values. Dialog-owned UI copy is Simplified Chinese, overriding English wording parity with Photoshop; technical identifiers, shortcuts and user data stay unchanged.

While any dialog is open, `AppState::modal_open()` is true: all menu commands are disabled and single-key shortcuts do not trigger, matching the behavior of Photoshop's modal dialogs.
