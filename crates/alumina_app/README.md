# egui demo app
This app demonstrates [`egui`](https://github.com/emilk/egui/) and [`eframe`](https://github.com/emilk/egui/tree/master/crates/eframe).

View the demo app online at <https://egui.rs>.

Run it locally with `cargo run --release -p alumina_app`.

`alumina_app` can be compiled to WASM and viewed in a browser locally with:

```sh
./scripts/start_server.sh &
./scripts/build_demo_web.sh --open
```

`alumina_app` uses [`egui_demo_lib`](https://github.com/emilk/egui/tree/master/crates/egui_demo_lib).


## Running with `wgpu` backend
`(cd alumina_app && cargo r --features wgpu)`
