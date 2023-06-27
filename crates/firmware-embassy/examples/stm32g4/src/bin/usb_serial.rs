#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use defmt::{panic, *};
use embassy_executor::Spawner;
use embassy_stm32::rcc::{ClockSrc, Pll, PllM, PllN, PllQ, PllR, PllSrc};
use embassy_stm32::time::Hertz;
use embassy_stm32::usb::{self, Driver, Instance};
use embassy_stm32::{bind_interrupts, pac, peripherals, Config};
use embassy_usb::class::cdc_acm::{CdcAcmClass, State};
use embassy_usb::driver::EndpointError;
use embassy_usb::Builder;
use futures::future::join;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USB_LP => usb::InterruptHandler<peripherals::USB>;
});

#[embassy_executor::main]
async fn main(_spawner: Spawner) {
    let mut config = Config::default();

    config.rcc.pll = Some(Pll {
        source: PllSrc::HSE(Hertz(8000000)),
        prediv_m: PllM::Div2,
        mul_n: PllN::Mul72,
        div_p: None,
        // USB and CAN at 48 MHz
        div_q: Some(PllQ::Div6),
        // Main system clock at 144 MHz
        div_r: Some(PllR::Div2),
    });

    config.rcc.mux = ClockSrc::PLL;

    let p = embassy_stm32::init(config);
    info!("Hello World!");

    pac::RCC.ccipr().write(|w| w.set_clk48sel(0b10));

    let driver = Driver::new(p.USB, Irqs, p.PA12, p.PA11);

    let mut config = embassy_usb::Config::new(0xc0de, 0xcafe);
    config.manufacturer = Some("Embassy");
    config.product = Some("USB-Serial Example");
    config.serial_number = Some("123456");

    config.device_class = 0xEF;
    config.device_sub_class = 0x02;
    config.device_protocol = 0x01;
    config.composite_with_iads = true;

    let mut device_descriptor = [0; 256];
    let mut config_descriptor = [0; 256];
    let mut bos_descriptor = [0; 256];
    let mut control_buf = [0; 64];

    let mut state = State::new();

    let mut builder = Builder::new(
        driver,
        config,
        &mut device_descriptor,
        &mut config_descriptor,
        &mut bos_descriptor,
        &mut control_buf,
    );

    let mut class = CdcAcmClass::new(&mut builder, &mut state, 64);

    let mut usb = builder.build();

    let usb_fut = usb.run();

    let echo_fut = async {
        loop {
            class.wait_connection().await;
            info!("Connected");
            let _ = echo(&mut class).await;
            info!("Disconnected");
        }
    };

    join(usb_fut, echo_fut).await;
}

struct Disconnected {}

impl From<EndpointError> for Disconnected {
    fn from(val: EndpointError) -> Self {
        match val {
            EndpointError::BufferOverflow => panic!("Buffer overflow"),
            EndpointError::Disabled => Disconnected {},
        }
    }
}

async fn echo<'d, T: Instance + 'd>(class: &mut CdcAcmClass<'d, Driver<'d, T>>) -> Result<(), Disconnected> {
    let mut buf = [0; 64];
    loop {
        let n = class.read_packet(&mut buf).await?;
        let data = &buf[..n];
        info!("data: {:x}", data);
        class.write_packet(data).await?;
    }
}
