# Alumina - a firmware and user interface for CNC and more

Todo:
- [ ] read boot button as input and display status
- [ ] parse self.url and use host portion to address microcontroller instead of hard coded uri
- [ ] calculate gcd in ui and firmware planners
- [ ] generate steps in stepper driver interrupt handler on micro
- [ ] implement queue between planner and stepper driver interrupt handler
- [ ] implement rate limiting in UI for geometry send
- [ ] implement SD support in firmware
- [ ] FAT / exFAT support for SD cards in firmware
- [ ] browse SD over HTTP in firmware
- [ ] UI for browsing SD in GUI
- [ ] read and display ADC values from micro
- [ ] read and display timer from micro
- [ ] read http headers, especially accept-encoding and content-length and others.
- [ ] integrate arcfinder
- [x] get 2D image viewer based on a bitmapped memory region working in GUI
- [ ] get 3d viewer module based on kiss3d / miniquad working for CAD
- [ ] implement ramp_maker in firmware from https://github.com/braun-embedded/ramp-maker/blob/main/examples/basic.rs
- [ ] implement setup mode, listening for esp direct communication of setup parameters and state machine around when to enable / disable setup mode
- [ ] implement wifi AP mode, and pull credentials into main.rs based on https://github.com/esp-rs/esp-idf-svc/blob/master/examples/json_post_handler.rs
- [ ] simplify and cargo-ize build system
- [x] implement https://github.com/101arrowz/fzstd as polyfill for wasm compression
- [ ] https://github.com/rustwasm/wasm-bindgen/pull/2000 WebXR bindings for WASM
- [ ] implement ETag / if-none-match for wasm based on build ( https://developer.mozilla.org/en-US/docs/Web/HTTP/Caching#etagif-none-match )
- [ ] enable https in firmware / UI with pinned cert, maybe still load UI via HTTP
- [ ] implement relay board support
- [ ] implement single axis board support
- [ ] implement 4 axis board support
- [ ] implement 4 axis c-6 board support

## Goals:
- Easy setup
- Networked and secure
- Modular
- High quality motion synchronization for steppers and three phase servo motors
- Broad support for MCUs of different models and from different vendors
- Zero memory errors (Buffer Overflow is the most common CVE for SCADA systems according to [Industrial Control
  Systems Vulnerabilities
  Statistics](https://media.kasperskycontenthub.com/wp-content/uploads/sites/43/2016/07/07190426/KL_REPORT_ICS_Statistic_vulnerabilities.pdf))
- Plasma, Laser, Router, Mill, Operator, Building support
- Versioned interfaces with feature discovery
## Status:
Initial implementation for host and MCU
## Application:
- High security and mission critical infrastructure automation
- Internet connected automation
## License(s):
MIT / BSD / Apache
## Underlying concepts:
- [English Metric Units](https://startbigthinksmall.wordpress.com/2010/01/04/points-inches-and-emus-measuring-units-in-office-open-xml/)
  EMUs are used for coordinates in vector, raster, and voxel space. The EMU is a virtual unit to bridge centimeters, inches, and points. One inch equates to 914400 EMUs and a centimeter is 360000. The number 914400 is calculated by the least common multiples: 100 and 254, times 72. This ensures that we can convert back and forth between integer 100th inches, millimeters and pixels without any rounding errors.
- [A Geometrical Understanding of Matrices](https://gregorygundersen.com/blog/2018/10/24/matrices/)
- [Shape Interrogation for Computer Aided Design and Manufacturing](https://web.mit.edu/hyperbook/Patrikalakis-Maekawa-Cho/)
- [Klipper's](https://www.klipper3d.org/Overview.html) multi-MCU messaging [protocol](https://www.klipper3d.org/Protocol.html) and [MCU commands](https://www.klipper3d.org/MCU_Commands.html)

## Development on Fedora 36 requires the following:
### Dependencies
```
sudo dnf groupinstall "C Development Tools and Libraries" "Development Tools"
sudo dnf install libudev-devel glib-devel gdk-pixbuf2-devel atk-devel cairo-devel pango-devel gtk3-devel
sudo ln -s /usr/lib/gcc/x86_64-redhat-linux/12/include/stdarg.h /usr/include/stdarg.h
sudo ln -s /usr/lib/gcc/x86_64-redhat-linux/12/include/stddef.h /usr/include/stdbdef.h
sudo ln -s /usr/lib/gcc/x86_64-redhat-linux/12/include/stdbool.h /usr/include/stdbool.h
cargo install cargo-flash
cargo install probe-run
cargo install flip-link
cargo install cargo-embed
rustup target install thumbv6m-none-eabi
rustup target install arm-unknown-linux-gnueabihf
rustup target install x86_64-pc-windows-gnu
rustup target install x86_64-apple-darwin
rustup default nightly
```
### Building
#### Linux
```
cargo build --release
```
#### Windows
```
cargo build --target x86_64-pc-windows-gnu --release
```
#### MacOS
follow osxcross setup [here](https://wapl.es/rust/2019/02/17/rust-cross-compile-linux-to-macos.html)
## Modules
### Algorithms
- GJK ([Youtube](https://www.youtube.com/watch?v=ajv46BSqcK4))
- CSG.js - difference, union, intersection in 2D, 3D
- primitive shapes in 2D, 3D: point, line, triangle, prism, sphere, cylinder / cone
- [curve tesselation](https://docs.rs/lyon_geom/latest/lyon_geom/)
- calculate normal vector of line, angle, triangle
- identify inside and outside of closed polylines (i.e. part and waste)
    - https://en.wikipedia.org/wiki/Straight_skeleton
    - https://doc.cgal.org/latest/Straight_skeleton_2/index.html
- shape offsetting
    - https://crates.io/crates/contour
    - https://crates.io/crates/cavalier_contours
    - https://raphlinus.github.io/curves/2022/09/09/parallel-beziers.html (from here: https://news.ycombinator.com/item?id=32784491)
- arbitrary 2D shape packing, shared surfaces
    - https://crates.io/crates/space-filling
    - https://github.com/prusa3d/PrusaSlicer/tree/master/src/libnest2d
    - https://github.com/tamasmeszaros/libnest2d
    - https://github.com/Jack000/SVGnest
    - https://github.com/Yisaer/Nest4J
    - https://www.sciencedirect.com/science/article/abs/pii/S0377221706001639
    - https://www.cs.stir.ac.uk/~goc/papers/EffectiveHueristic2DAOR2013.pdf
    - https://github.com/MasumBhuiyan/2D-Irregular-Cutting-Stock-Algorithm
    - https://github.com/Pseudomanifold/bin-packing-heuristics
    - https://github.com/whitegreen/Dalsoo-Bin-Packing
    - https://github.com/Pseudomanifold/bin-packing-heuristics
    - https://github.com/cicirello/InteractiveBinPacking
    - https://github.com/alikhanlab/wirecut-optimization
    - https://www.sciencedirect.com/science/article/pii/S240589631930864X
    - https://ieeexplore.ieee.org/document/8796818
    - https://iopscience.iop.org/article/10.1088/1742-6596/2181/1/012002/meta
    - https://www.researchgate.net/publication/330705071_Obstruction_map_local_search_solution_for_2D_irregular_bin_packing_problem_with_cache_acceleration
    - https://www.researchgate.net/publication/362605146_Two-dimensional_irregular_packing_problems_A_review/fulltext/62f3d8e979550d6d1c6fcf99/Two-dimensional-irregular-packing-problems-A-review.pdf?origin=publication_detail
    - https://www.faqgit.com/repositories/albert-espin/knapsack-packing?id=85733
- path optimization (distance, spindle load, tolerance, velocity, temperature, etc)
    - https://crates.io/crates/optimization_engine
    - https://crates.io/crates/gomez
    - https://crates.io/crates/newton_rootfinder
    - https://crates.io/crates/levenberg-marquardt
    - https://github.com/djrakita/optima_toolbox
- [motion ramping](https://crates.io/crates/ramp-maker)
- [motion smoothing](https://www.klipper3d.org/Kinematics.html) using a lookahead window
- contour tracing from raster to vector
    - [raster2svg](https://crates.io/crates/raster2svg) uses the [contour_tracing](https://crates.io/crates/contour_tracing) library.
    - [marching squares](https://crates.io/crates/marching-squares) creates contour lines from a heightmap of Vec\<Vec\<i16\>\>
- [coordinate transformations](https://en.wikipedia.org/wiki/List_of_common_coordinate_transformations)
    - https://github.com/DaveKram/coord_transforms
### Components
- timer
    - https://github.com/etrombly/bluepill/blob/master/examples/stepper_tasks.rs
- analog to digital converter
- encoders
    - https://crates.io/crates/qei
    - https://crates.io/crates/rotary-encoder-hal
- command processor
- interrupt handler
- GPIO pin
- Wiznet W5500
    - https://crates.io/crates/w5500
    - https://crates.io/crates/w5500-hl
    - https://crates.io/crates/w5500-dhcp
    - https://crates.io/crates/w5500-dns
    - https://crates.io/crates/w5500-ll
    - https://crates.io/crates/w5500-mqtt
    - https://crates.io/crates/w5500-sntp
    - https://crates.io/crates/w5500-tls
    - https://crates.io/crates/w5500-http
    - https://crates.io/crates/w5500-https
    - https://crates.io/crates/w5500-ntp
    - https://crates.io/crates/w5500-regsim
- ENC28J60
    - https://crates.io/crates/enc28j60
    - http://blog.japaric.io/wd-4-enc28j60/
- ESP32
    - https://github.com/esp-rs/rust
    - https://github.com/esp-rs/esp-idf-hal
    - https://github.com/esp-rs/esp-hal
    - https://github.com/MabezDev/xtensa-rust-quickstart
- [accellerometer](https://github.com/NeoBirth/accelerometer.rs)
- [3 phase driver (triple H bridge)]
- [stepper driver (dual H bridge)]
- [isolation ICs]
- [camera IC]
- [mems mic]
- [relay / SSR]
- voltage regulator(s)
- ethernet jack (w/ or w/o integrated coils - poe)
- passives (caps, resistors, diodes, fuses)
- connectors (wago)
- [SD / EMMC](https://github.com/rust-embedded-community/embedded-storage)
- [stepper motors](https://crates.io/crates/stepper)
    - https://github.com/etrombly/bluepill/blob/master/examples/stepper_tasks.rs
- three phase servo motors
### Circuits
- Full H bridge with high and low side driver control
- [current sensing](https://www.analog.com/en/app-notes/an-105fa.html)
- voltage sensing
- isolation for GPIO, ADC, I2C, SPI, RS232/422/485, etc
- buck / boost
- PoE power supply (up to 48v, 100W - would be nice to push to non-standard 400W)
- relay / SSR driver
- ethernet to SPI adapter using w5500
### Boards (PoE + positive locking terminal connector)
- low, mid, high power three phase motor controller / inverter / vfd for open and closed loop w/ GPIO
- low, mid power stepper controller for open and closed loop w/ GPIO
- toolhead controller w/ serial / modbus, thc, capacitive sensing, accellerometer
- torch / laser / vfd controller
- dc-dc mppt power conditioner for solar panels up to 400W (MPPT PoE sender?)
- 110/220v remote controlled receptical
- multi-chemistry battery management system
- remote sensors / actuators: NIR, temp / humidity, RFID, accellerometer, PIN pad, deadbolt, solenoid, liquid and gas flow, current, voltage,
- camera, mic, audio output
- PoE network motion controller
### Axes
- Actuator(s)
- Sensor(s)
- Coordinate space (cartesian, spherical, polar, hyperbolic, etc)
- Relationship between actuator and movement in coordinate space in EMUs
- Basic CAD representation built-in
### Machines
- Axes in coordinate space relationships to each other
- control loops
- communication
- synchronization
- consumables
### Locations
- Machines
- Operators
- Security, safety, and authorization
- Coordination

## Initial setup should be via Wifi / Bluetooth / QR Code lasered onto device
- log into app / web app / control backplane
- add public key of device to control backplane (via QR code)
- add public key of control backplane to device (via Wifi / BT)
- each device can only contain one control plane key?
- device connects to control backplane https://github.com/rapiz1/rathole and starts receiving encrypted messages
- https://github.com/bluez/bluer
- https://wiki.pine64.org/wiki/PineCone
- https://github.com/jonas-schievink/rubble
- https://crates.io/crates/chacha20poly1305

## Host:
### File inputs: 2D geometry in files formatted as:
- SVG
    - https://crates.io/crates/geo-svg-io
    - https://github.com/RazrFalcon/resvg
- DXF
    - https://crates.io/crates/dxf2image
    - https://www.loc.gov/preservation/digital/formats/fdd/fdd000446.shtml
    - Test data: https://github.com/gdsestimating/dxf-parser/tree/master/test/data
- GCode
    - https://tsapps.nist.gov/publication/get_pdf.cfm?pub_id=823374
    - https://github.com/zethra/gcode-rs
    - https://github.com/Michael-F-Bryan/gcode-rs
    - https://reprap.org/wiki/G-code
    - https://en.wikipedia.org/wiki/G-code
    - https://en.wikipedia.org/wiki/Gerber_format#RS-274X
    - https://www.ucamco.com/en/gerber
    - https://lib.rs/crates/nom-reprap-response
    - https://crates.io/crates/grbli
- Bitmaps / rasters

### File inputs: 3D geometry in files formatted as:
- STL
- STEP / IGES - https://github.com/ricosjp/ruststep
- Blender

### We may have to fix up the data:
- for 2D and 3D:
    - merge points closer than some minimum distance
    - interpolate arcs and curves, otherwise tesselate complex shapes
    - sort points and lines by connectedness
- for 3D:
    - fix inverted normals
    - detect and fix manifoldness

### We stuff the data into [nalgebra types](https://docs.rs/nalgebra/latest/nalgebra/geometry/index.html)
- points: tuples of integer values representing coordinates in X, Y, Z axes in English Metric Units
- polylines: ordered list or vec of points
    - will be useful to have an iterator over each set of polylines
    - perhaps useful to keep connected polylines in seperate buckets for various ops, iterator over connected polyline groups
- triangles: tuples of point tuples, normal vector encoded in winding order
- Ideally all of these are organized as an array or vec of references into another vec of points.  Or some other memory efficient arrangement.
- https://crates.io/crates/geo/
- https://crates.io/crates/geo-booleanop/
- https://github.com/georust

### For 3D toolpathing operations, this means we will need an idea of the shape of the working material:
- 

### Our toolpath must include lead-ins and lead-outs:
- 

### We may need to do trigonometric functions on fixed point numbers quickly:
- https://github.com/sebcrozet/cordic

### We may need to turn shapes into triangles for the GPU:
- https://github.com/nical/lyon
- https://github.com/pcwalton/pathfinder
- http://kiss3d.org/
- https://crates.io/crates/earcutr

### We will translate the toolpath into commands for the MCU:
- https://www.klipper3d.org/Protocol.html
- https://www.klipper3d.org/MCU_Commands.html
- http://linuxcnc.org/docs/2.5/html/code/NML_Messages.html
- http://linuxcnc.org/docs/stable/html/config/python-interface.html
- https://crates.io/crates/linuxcnc-hal

### We need buffered (queued), and unbuffered (instantaneous) commands:
- Buffered:
    - Most moves
- Unbuffered:
    - Estop (both directions)
    - Jog (from pendant, to motors)
    - Torch Height Control

### We will calculate the timer offsets from system time for each of our multiple microcontrollers so that we can accurately timestamp each command for execution:
- https://github.com/Klipper3d/klipper/blob/master/klippy/clocksync.py

### And transport them to the MCU via an encrypted serial or network protocol:
- https://github.com/oxidecomputer/tlvc
- https://github.com/oxidecomputer/rusb
- https://crates.io/crates/dryoc
- https://github.com/smoltcp-rs/smoltcp
- https://github.com/tokio-rs/tokio
- https://github.com/berkowski/tokio-serial
- https://subscription.packtpub.com/book/application-development/9781788399487/11/ch11lvl1sec51/asynchronous-i-o-in-rust
- https://crates.io/crates/defmt
- https://crates.io/crates/defmt-serial
- https://gitlab.com/susurrus/serialport-rs
- https://lib.rs/crates/net-serial-console
- https://github.com/zethra/cinder
- https://crates.io/crates/embedded-websocket
- https://github.com/drogue-iot/embedded-tls
- https://github.com/smoltcp-rs/smoltcp

## MCU:
### Firmwares:
- https://github.com/zethra/wasp
- https://github.com/zethra/wasp_teensy32
- https://github.com/mdtusz/wasp
- https://github.com/rust-embedded/embedded-hal
- https://github.com/jonlamb-gh/oxcc
### Hardware support:
- https://crates.io/crates/embedded-graphics
- https://github.com/rust-iot/radio-hal
- https://github.com/smart-leds-rs
- https://github.com/rust-embedded-community/usb-device
- https://github.com/rust-embedded-community/embedded-nal
- https://crates.io/crates/cam-geom
- https://crates.io/crates/device-driver
- https://crates.io/crates/bitbang-hal
- https://crates.io/crates/ftdi-embedded-hal
### Embedded software implementations:
- https://crates.io/crates/bounded-registers
- https://crates.io/crates/tock-registers
- https://crates.io/crates/adskalman
- https://crates.io/crates/atomic
- https://crates.io/crates/bbqueue
- https://crates.io/crates/debouncr
- https://crates.io/crates/embedded-crc-macros
- https://crates.io/crates/heapless
- https://crates.io/crates/irq
- https://github.com/tarcieri/micromath
- https://crates.io/crates/num-format
- https://crates.io/crates/scapegoat
### OTA Updates:
- https://github.com/drogue-iot/embedded-update
- https://github.com/jhbruhn/moonboot
### Bitbanging VGA:
- https://github.com/thejpster/vga-framebuffer-rs

### Our MCU will need an events system (interrupt based, polling, etc):
- Monitor temperature sensors, encoders, estops, and various other inputs
- Register a configurable action to execute on event
- Register a configurable time to trigger the event (optionally repeating)
- Commands to configure all this
- https://github.com/drogue-iot/ector

### Our MCU will have to generate timed pulses for step/dir, PWM, I/O, etc:

### We will sometimes have to speak other protocols to peripherals, necessitating commands and MCU facilities for doing so:
- https://github.com/slowtec/tokio-modbus
- https://github.com/locka99/opcua

### Web API:
- https://salvo.rs/

### Browser extensions:
- https://github.com/Mubelotix/wasm-extension-template

### Database:
- https://surrealdb.com/
- Machines
    - Machine ID
    - Machine type
    - Machine owner
    - Date of manufacture
    - Machine on hours
    - EMUs moved, each axis
- Services
    - Time of service
    - Type of service
    - Tool serviced
- People
    - First name
    - Last name
    - Shipping address
    - Billing address
- Toolpaths
    - Unique ID
    - Polylines - https://crates.io/crates/polyline ?
    - Project Name

## Resources:
### General:
- https://rust-unofficial.github.io/patterns/intro.html
- https://en.wikipedia.org/wiki/SCADA
- https://github.com/rust-embedded/awesome-embedded-rust
- https://github.com/armyofevilrobots/aoer-plotty-rs
- https://github.com/scottalford75/Remora
- https://mecatronyx.gitlab.io/opencnc/opencn/CNC_Path_Planning_Algorithms/Geometric_Operations/Geometric_Operations.html
- https://github.com/distrap/lambdadrive/
- https://github.com/orgs/distrap/repositories
- http://www.hashmismatch.net/pragmatic-bare-metal-rust/
- http://blog.japaric.io/fearless-concurrency/
- https://docs.rust-embedded.org/discovery/index.html
- https://blend2d.com/research/simplify_and_offset_bezier_curves.pdf
- [Bravais lattices](https://en.wikipedia.org/wiki/Bravais_lattice)
  In geometry and crystallography, a Bravais lattice is a category of translative symmetry groups (also known as lattices).  There are 5 Bravais lattices in two dimensions, 14 Bravais lattices in three dimensions, and 64 Bravais lattices in four dimensions. Of the 64, 23 are primitive and 41 are centered, 10 are split into enantiomorphic pairs.  All crystalline materials (not including quasicrystals) must, by definition, fit into one of these arrangements. For convenience a Bravais lattice is depicted by a unit cell which is a factor 1, 2, 3, or 4 larger than the primitive cell. Depending on the symmetry of a crystal or other pattern, the fundamental domain is again smaller, up to a factor 48.
### Testing:
- https://github.com/canndrew/netsim
### P2P:
- https://github.com/libp2p/rust-libp2p/blob/master/examples/file-sharing.rs