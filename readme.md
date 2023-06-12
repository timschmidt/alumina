To build:

install rust from rustup.rs
ui/scripts/setup_web.sh
install esp32c3 toolchain
ui/scripts/build_demo_web.sh --release
firmware-esp32c3: cargo run --release
