# titlebar.rs: Custom-drawn title bar (macOS)

## Simplified Chinese fork override

OpenPhoto-zh-CN displays the component's application-authored labels, operation names, prompts and errors in Simplified Chinese. The English names in the behavioral reference below identify the upstream commands, not the fork's literal display copy. Product names, format abbreviations, shortcuts, numerical formatting, internal IDs and serialization keys remain unchanged. Existing document titles, layer names, paths and other user data are never translated.

The title bar retains the product name OpenPhoto; it contains no translatable prose.


## Component responsibility

On macOS the window hides the system title bar (content extends over the whole window), so this draws a title bar in the same color as the options bar, making it look like Photoshop's window. The traffic-light buttons are still drawn by the system. Other platforms use the system title bar and do not show this component.

## Visuals

- 40 reference pixels high, background `#535353`.
- "OpenPhoto" is shown centered (Photoshop shows "Adobe Photoshop 2026").

## Interaction

- Press and drag: moves the window (`ViewportCommand::StartDrag`).
- Double-click: toggles between maximized and restored.
