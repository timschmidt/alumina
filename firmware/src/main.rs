#![no_main] // The no_main attribute indicates to the Rust compiler not to link to the Rust standard runtime. This is commonly used for bare-metal and embedded programming.
#![no_std] // The no_std attribute disables the standard library. This is commonly used in embedded systems where there may not be OS-level features that the standard library requires.

// This line imports the panic_halt crate to handle panics, i.e., unexpected program errors.
use panic_halt as _;

use cortex_m_rt::entry; // The cortex_m_rt crate provides runtime functionality for Cortex-M processors.
use stm32f4xx_hal as hal; // The stm32f4xx_hal crate provides a Hardware Abstraction Layer (HAL) for the STM32F4xx family of microcontrollers.

use crate::hal::delay::Delay; // Importing the Delay struct for creating delay functionalities.
use crate::hal::pac; // The pac module (Peripheral Access Crate) provides a low-level access to the microcontroller's peripherals.
use crate::hal::prelude::*; // The prelude module usually re-exports the most important parts of the crate to easily include them all at once.
use crate::hal::spi::Spi; // Importing the Serial Peripheral Interface (SPI) struct for serial communication.
use w5500_ll::{blocking::vdm::W5500, Registers}; // The w5500_ll crate provides low-level access to the WIZnet W5500 Ethernet chip.
use w5500_hl::ll::{net::{Ipv4Addr, SocketAddrV4}, Sn}; // The w5500_hl crate provides high-level access to the WIZnet W5500 Ethernet chip.
use w5500_hl::Tcp; // Importing the TCP protocol struct.

#[entry] // The entry attribute from the cortex_m_rt crate defines the entry point of the program.
fn main() -> ! { // The main function is the entry point of the program. The '!' return type indicates that this function will never return.
    // Attempt to take control of the microcontroller's peripherals.
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain the Reset and Clock Control (RCC) peripheral to freeze its configuration and get a handle to the clock configuration.
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();

        // Initialize a delay provider.
        let mut delay = Delay::new(cp.SYST, &clocks);

        // Split the GPIOA and GPIOC peripherals into their constituent pins.
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // Configure pin 13 of GPIOC to be a push-pull output and set it to low to turn on the onboard LED.
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // Configure pins for SPI communication.
        let cs = gpioa.pa4.into_push_pull_output(); // Chip select pin
        let sck1 = gpioa.pa5.into_alternate(); // Serial Clock pin
        let miso1 = gpioa.pa6.into_alternate(); // Master In Slave Out pin
        let mosi1 = gpioa.pa7.into_alternate(); // Master Out Slave In pin

        // Initialize the SPI1 peripheral with a frequency of 3 MHz.
        let spi = Spi::new(dp.SPI1, (sck1, miso1,mosi1), embedded_hal::spi::MODE_0, 3_000_000.hz(), clocks);

        // Initialize the W5500 Ethernet chip with the SPI interface and the chip select pin.
        let mut w5500 = W5500::new(spi, cs);
        // Get the version of the W5500 chip and assert that it's 0x04.
        let version: u8 = w5500.version().unwrap();
        assert_eq!(version, 0x04);

        // Define the socket number and port number for HTTP communication.
        const HTTP_SOCKET: Sn = Sn::Sn1;
        const HTTP_PORT: u16 = 80;

        // Instruct the W5500 chip to start listening for TCP connections on the defined socket and port.
        match w5500.tcp_listen(HTTP_SOCKET, HTTP_PORT) {
            Result => {} // If the tcp_listen function returns a Result, it's ignored.
            Option => {} // If the tcp_listen function returns an Option, it's also ignored.
        }
    }
    // A never-ending loop. In embedded systems, this is common as the program is supposed to keep running until the system is powered off.
    loop {
        continue;
    }
}
