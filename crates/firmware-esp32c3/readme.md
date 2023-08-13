To build:

cargo run --release

A new WASM file will be built, stripped, and gzip compressed within the egui directory and linked in to the build.

Todo:
- [ ] read boot button as input and display status
- [ ] parse self.url and use host portion to address microcontroller instead of hard coded uri
- [ ] calculate gcd in ui and firmware
- [ ] generate steps in stepper driver interrupt handler on micro
- [ ] implement rate limiting in UI for geometry send
- [ ] implement SD support
- [ ] browse SD over HTTP
- [ ] UI for browsing SD
- [ ] read and display ADC values from micro
- [ ] read and display timer from micro
- [ ] read http headers, especially accept-encoding and content-length and others.
- [ ] integrate arcfinder
- [ ] get image memory mapped UI working for image / voxel
- [ ] get kiss3d UI and render working for CAD
- [ ] implement ramp_maker in firmware from https://github.com/braun-embedded/ramp-maker/blob/main/examples/basic.rs