#![no_main] // The no_main attribute indicates to the Rust compiler not to link to the Rust standard runtime. This is commonly used for bare-metal and embedded programming.
#![no_std] // The no_std attribute disables the standard library. This is commonly used in embedded systems where there may not be OS-level features that the standard library requires.

use panic_halt as _; // This line imports the panic_halt crate to handle panics, i.e., unexpected program errors.
use cortex_m_rt::entry; // The cortex_m_rt crate provides runtime functionality for Cortex-M processors.
use stm32f4xx_hal as hal;
// The stm32f4xx_hal crate provides a Hardware Abstraction Layer (HAL) for the STM32F4xx family of microcontrollers.
//use crate::hal::delay::Delay; // Importing the Delay struct for creating delay functionalities.
use crate::hal::pac; // The pac module (Peripheral Access Crate) provides a low-level access to the microcontroller's peripherals.
use crate::hal::prelude::*; // The prelude module usually re-exports the most important parts of the crate to easily include them all at once.
use crate::hal::spi::Spi; // Importing the Serial Peripheral Interface (SPI) struct for serial communication.
use fugit::Rate; // no_std library for computing rates and durations at compile time
use w5500_hl::{ // The w5500_hl crate provides high-level access to the WIZnet W5500 Ethernet chip.
    ll::{blocking::vdm::W5500, Registers, Sn, SocketInterrupt},// The w5500_ll crate provides low-level access to the WIZnet W5500 Ethernet chip.
    net::{Ipv4Addr, SocketAddrV4},
    Tcp
};
use debouncr::debounce_4;
// global_allocator is currently avaliable on nightly for embedded rust
//extern crate alloc;
//use alloc::vec::{self, Vec};

#[entry] // The entry attribute from the cortex_m_rt crate defines the entry point of the program.
fn main() -> ! { // The main function is the entry point of the program. The '!' return type indicates that this function will never return.
    // Attempt to take control of the microcontroller's peripherals.
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain the Reset and Clock Control (RCC) peripheral to freeze its configuration and get a handle to the clock configuration.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(Rate::<u32, 1, 1>::from_raw(48_000_000)).freeze();

        // Initialize a delay provider.
        //let mut delay = Delay::new(cp.SYST, &clocks);

        // Split the GPIOA and GPIOC peripherals into their constituent pins.
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // Configure pin 13 of GPIOC to be a push-pull output and set it to low to turn on the onboard LED.
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // Configure pin 0 of GPIOA to be a pull-up input for the button.
        let button = gpioa.pa0.into_pull_up_input();
        let mut debouncer = debounce_4(false); // Type: Debouncer<u8, Repeat4>

        // Configure pins for SPI communication.
        let cs = gpioa.pa4.into_push_pull_output(); // Chip select pin
        let sck1 = gpioa.pa5.into_alternate(); // Serial Clock pin
        let miso1 = gpioa.pa6.into_alternate(); // Master In Slave Out pin
        let mosi1 = gpioa.pa7.into_alternate(); // Master Out Slave In pin

        // Initialize the SPI1 peripheral with a frequency of 3 MHz.
        let spi = Spi::new(dp.SPI1, (sck1, miso1, mosi1), embedded_hal::spi::MODE_0, Rate::<u32, 1, 1>::from_raw(3_000_000), &clocks);

        // Initialize the W5500 Ethernet chip with the SPI interface and the chip select pin.
        let mut w5500 = W5500::new(spi, cs);
        // Get the version of the W5500 chip and assert that it's 0x04.
        let version: u8 = w5500.version().unwrap();
        assert_eq!(version, 0x04);

        // Define the socket number and port number for HTTP communication.
        const HTTP_SOCKET: Sn = Sn::Sn0;
        const HTTP_PORT: u16 = 80;

        // Instruct the W5500 chip to start listening for TCP connections on the defined socket and port.
        let connection  = w5500.tcp_listen(HTTP_SOCKET, HTTP_PORT);

        // wait for the RECV interrupt, indicating there is data to read from a client
        loop {
            let sn_ir = w5500.sn_ir(HTTP_SOCKET).unwrap();
            if sn_ir.recv_raised() {
                w5500.set_sn_ir(HTTP_SOCKET, sn_ir).unwrap();

                // Read the received data
                let mut buf: [u8; 256] = [0; 256];
                let rx_bytes: u16 = w5500.tcp_read(HTTP_SOCKET, &mut buf).unwrap();

                // Convert the received bytes to a string
                let received_message = core::str::from_utf8(&buf[..rx_bytes as usize]).unwrap();

                // Check if the received message matches the expected message
                if received_message == "Button pressed" {
                    // Toggle the LED
                    led.toggle();
                }
            }
            if sn_ir.discon_raised() | sn_ir.timeout_raised() {
                panic!("Socket disconnected while waiting for RECV");
            }

            // Read the button state and update the debouncer
            let button_state = button.is_low();
            if let Some(edge) = debouncer.update(button_state) {
                match edge {
                    debouncr::Edge::Rising => {
                        // Handle button press
                        led.toggle();

                        // Send a message over the network
                        let message = "Button pressed";
                        if let Err(err) = send_message(HTTP_SOCKET, message, &mut w5500) {
                            panic!("Failed to send message: {:?}", err);
                        }
                    }
                    debouncr::Edge::Falling => {
                        // Handle button release
                    }
                }
            }
        }

        //let mut buf: Vec<u8> = vec![0; 256];
        //let rx_bytes: u16 = w5500.tcp_read(HTTP_SOCKET, &mut buf)?;
        // Truncate the buffer to the number of bytes read
        // Safety: BUF is only borrowed mutably in one location
        //let filled_buf: &[u8] = &buf[..rx_bytes.into()];

        // parse HTTP request here using filled_buf
    }
    else { panic!("Unable to take control of microcontroller peripherals") }
}

fn send_message<SPI, CS>(
    socket: Sn,
    message: &str,
    w5500: &mut W5500<SPI, CS>
) -> Result<(), w5500_hl::ll::blocking::vdm::Error<SPI::Error, CS::Error>>
    where
        SPI: embedded_hal::spi::FullDuplex<u8>,
        CS: embedded_hal::digital::v2::OutputPin,
{
    let bytes = message.as_bytes();
    w5500.tcp_write(socket, bytes)
}