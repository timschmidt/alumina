# Architecture
This document describes how the crates that make up egui are all connected.

Also see [`CONTRIBUTING.md`](CONTRIBUTING.md) for what to do before opening a PR.


## Crate overview
The crates in this repository are: `egui, emath, epaint, egui_extras, egui-winit, egui_glium, egui_glow, egui_demo_lib, egui_demo_app`.

### `egui`: The main GUI library.
Example code: `if ui.button("Click me").clicked() { … }`
This is the crate where the bulk of the code is at. `egui` depends only on `emath` and `epaint`.

### `emath`: minimal 2D math library
Examples: `Vec2, Pos2, Rect, lerp, remap`

### `epaint`
2d shapes and text that can be turned into textured triangles.

Example: `Shape::Circle { center, radius, fill, stroke }`

Depends on `emath`.

### `egui_extras`
This adds additional features on top of `egui`.

### `egui-winit`
This crates provides bindings between [`egui`](https://github.com/emilk/egui) and [winit](https://crates.io/crates/winit).

The library translates winit events to egui, handled copy/paste, updates the cursor, open links clicked in egui, etc.

### `egui_glium`
Puts an egui app inside a native window on your laptop. Paints the triangles that egui outputs using [glium](https://github.com/glium/glium).

### `egui_glow`
Puts an egui app inside a native window on your laptop. Paints the triangles that egui outputs using [glow](https://github.com/grovesNL/glow).

### `eframe`
`eframe` is the official `egui` framework, built so you can compile the same app for either web or native.

The demo that you can see at <https://www.egui.rs> is using `eframe` to host the `egui`. The demo code is found in:

### `egui_demo_lib`
Depends on `egui`.
This contains a bunch of uses of `egui` and looks like the ui code you would write for an `egui` app.

### `alumina_app`
Thin wrapper around `egui_demo_lib` so we can compile it to a web site or a native app executable.
Depends on `egui_demo_lib` + `eframe`.

### Other integrations

There are also many great integrations for game engines such as `bevy` and `miniquad` which you can find at <https://github.com/emilk/egui#integrations>.
