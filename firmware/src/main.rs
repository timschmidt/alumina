#![no_main]
#![no_std]

// Halt on panic
use panic_halt as _; // panic handler

use cortex_m_rt::entry;
use stm32f4xx_hal as hal;

use crate::hal::delay::Delay;
use crate::hal::pac;
//use crate::hal::gpio::NoPin;
use crate::hal::prelude::*;
use crate::hal::spi::Spi;
use w5500_ll::{blocking::vdm::W5500, Registers};
use w5500_hl::ll::{net::{Ipv4Addr, SocketAddrV4}, Sn};
use w5500_hl::Tcp;

#[entry]
fn main() -> ! {
    if let (Some(dp), Some(cp)) = (
        pac::Peripherals::take(),
        cortex_m::peripheral::Peripherals::take(),
    ) {
        // Constrain clocking registers
        let rcc = dp.RCC.constrain();
        let clocks = rcc.cfgr.sysclk(48.mhz()).freeze();
        
        let mut delay = Delay::new(cp.SYST, &clocks);

        // GPIOA used for SPI, GPIOC for onboard led
        let gpioa = dp.GPIOA.split();
        let gpioc = dp.GPIOC.split();

        // turn onboard led on
        let mut led = gpioc.pc13.into_push_pull_output();
        led.set_low();

        // Configure pins for SPI
        let cs = gpioa.pa4.into_push_pull_output();
        let sck1 = gpioa.pa5.into_alternate();
        let miso1 = gpioa.pa6.into_alternate();
        let mosi1 = gpioa.pa7.into_alternate();

        // SPI1 with 3Mhz
        let spi = Spi::new(dp.SPI1, (sck1, miso1, mosi1), embedded_hal::spi::MODE_0, 3_000_000.hz(), clocks);

        let mut w5500 = W5500::new(spi, cs);
        let version: u8 = w5500.version().unwrap();
        assert_eq!(version, 0x04);

        const HTTP_SOCKET: Sn = Sn::Sn1;
        const HTTP_PORT: u16 = 80;

        // serve HTTP
        match w5500.tcp_listen(HTTP_SOCKET, HTTP_PORT) {
            Result => {}
            Option => {}
        }

    }
    loop {


        continue;
    }
}