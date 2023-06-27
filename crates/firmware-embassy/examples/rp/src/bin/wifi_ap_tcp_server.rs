#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]
#![feature(async_fn_in_trait)]
#![allow(incomplete_features)]

extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use core::str::from_utf8;

use cyw43_pio::PioSpi;
use defmt::*;
use embassy_executor::Spawner;
use embassy_net::tcp::TcpSocket;
use embassy_net::{Config, Stack, StackResources};
use embassy_rp::gpio::{Level, Output};
use embassy_rp::peripherals::{DMA_CH0, PIN_23, PIN_25, PIO0};
use embassy_rp::pio::Pio;
use embassy_time::Duration;
use embedded_io::asynch::Write;
use static_cell::make_static;
use {defmt_rtt as _, panic_probe as _};
use httparse::{Request, EMPTY_HEADER};

#[embassy_executor::task]
async fn wifi_task(
    runner: cyw43::Runner<'static, Output<'static, PIN_23>, PioSpi<'static, PIN_25, PIO0, 0, DMA_CH0>>,
) -> ! {
    runner.run().await
}

#[embassy_executor::task]
async fn net_task(stack: &'static Stack<cyw43::NetDriver<'static>>) -> ! {
    stack.run().await
}

#[embassy_executor::main]
async fn main(spawner: Spawner) {
    info!("Hello World!");

    let p = embassy_rp::init(Default::default());

    let fw = include_bytes!("../../../../cyw43-firmware/43439A0.bin");
    let clm = include_bytes!("../../../../cyw43-firmware/43439A0_clm.bin");

    // To make flashing faster for development, you may want to flash the firmwares independently
    // at hardcoded addresses, instead of baking them into the program with `include_bytes!`:
    //     probe-rs-cli download 43439A0.bin --format bin --chip RP2040 --base-address 0x10100000
    //     probe-rs-cli download 43439A0_clm.bin --format bin --chip RP2040 --base-address 0x10140000
    //let fw = unsafe { core::slice::from_raw_parts(0x10100000 as *const u8, 224190) };
    //let clm = unsafe { core::slice::from_raw_parts(0x10140000 as *const u8, 4752) };

    let pwr = Output::new(p.PIN_23, Level::Low);
    let cs = Output::new(p.PIN_25, Level::High);
    let mut pio = Pio::new(p.PIO0);
    let spi = PioSpi::new(&mut pio.common, pio.sm0, pio.irq0, cs, p.PIN_24, p.PIN_29, p.DMA_CH0);

    let state = make_static!(cyw43::State::new());
    let (net_device, mut control, runner) = cyw43::new(state, pwr, spi, fw).await;
    unwrap!(spawner.spawn(wifi_task(runner)));

    control.init(clm).await;
    control
        .set_power_management(cyw43::PowerManagementMode::PowerSave)
        .await;

    // Use a link-local address for communication without DHCP server
    let config = Config::ipv4_static(embassy_net::StaticConfigV4 {
        address: embassy_net::Ipv4Cidr::new(embassy_net::Ipv4Address::new(169, 254, 1, 1), 16),
        dns_servers: heapless::Vec::new(),
        gateway: None,
    });

    // Generate random seed
    let seed = 0x0123_4567_89ab_cdef; // chosen by fair dice roll. guarenteed to be random.

    // Init network stack
    let stack = &*make_static!(Stack::new(
        net_device,
        config,
        make_static!(StackResources::<2>::new()),
        seed
    ));

    unwrap!(spawner.spawn(net_task(stack)));

    //control.start_ap_open("cyw43", 5).await;
    control.start_ap_wpa2("replimat", "replimat", 5).await;

    // And now we can use it!

    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];
    let mut n = 0;

    let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
    socket.set_timeout(Some(Duration::from_secs(10)));

    control.gpio_set(0, false).await;
    info!("Listening on TCP:80...");
    if let Err(e) = socket.accept(80).await {
        warn!("accept error: {:?}", e);
        continue;
    }

    info!("Received connection from {:?}", socket.remote_endpoint());
    control.gpio_set(0, true).await;

    loop {
        match socket.read(&mut buf[n..]).await {
            Ok(0) => {
                warn!("read EOF");
                break;
            }
            Ok(read_bytes) => {
                n += read_bytes;
            }
            Err(e) => {
                warn!("read error: {:?}", e);
                break;
            }
        }

        let mut req_headers = [EMPTY_HEADER; 16];
        let mut req = httparse::Request::new(&mut req_headers);
        if let Ok(_) = req.parse(&buf[..n]) {
            info!("rxd {}", from_utf8(&buf[..n]).unwrap());

            let method = req.method.unwrap();
            let path = req.path.unwrap();

            if method != "GET" {
                let response = "HTTP/1.1 405 Method Not Allowed\r\n\r\n";
                socket.write_all(response.as_bytes()).await.unwrap();
                break;
            }

            match path {
                "/" => {
                    let content = include_bytes!("../../../../../egui/docs/index.html.gz");
                    let response = ["HTTP/1.1 200 OK\r\nContent-Length: ".as_bytes(), &content.len().to_string().as_bytes(), "\r\n\r\n".as_bytes(), content].concat();

                    // Send response in chunks of 1024 bytes
                    let mut start = 0;
                    while start < response.len() {
                        let end = start + 1024; // Change this to match your MSS, minus protocol headers
                        let end = end.min(response.len());

                        match socket.write_all(&response[start..end]).await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!("write error: {:?}", e);
                                break;
                            }
                        };

                        start = end;
                    }

                },
                "/index.js" => {
                    let content = include_bytes!("../../../../../egui/docs/index.js.gz");
                    let response = ["HTTP/1.1 200 OK\r\nContent-Length: ".as_bytes(), &content.len().to_string().as_bytes(), "\r\n\r\n".as_bytes(), content].concat();

                    // Send response in chunks of 1024 bytes
                    let mut start = 0;
                    while start < response.len() {
                        let end = start + 1024; // Change this to match your MSS, minus protocol headers
                        let end = end.min(response.len());

                        match socket.write_all(&response[start..end]).await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!("write error: {:?}", e);
                                break;
                            }
                        };

                        start = end;
                    }
                },
                "/index.wasm" => {
                    let content = include_bytes!("../../../../../egui/docs/index_bg.wasm.gz");
                    let response = ["HTTP/1.1 200 OK\r\nContent-Length: ".as_bytes(), &content.len().to_string().as_bytes(), "\r\n\r\n".as_bytes(), content].concat();

                    // Send response in chunks of 1024 bytes
                    let mut start = 0;
                    while start < response.len() {
                        let end = start + 1024; // Change this to match your MSS, minus protocol headers
                        let end = end.min(response.len());

                        match socket.write_all(&response[start..end]).await {
                            Ok(()) => {}
                            Err(e) => {
                                warn!("write error: {:?}", e);
                                break;
                            }
                        };

                        start = end;
                    }
                },
                _ => {
                    let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                    socket.write_all(response.as_bytes()).await.unwrap();
                    break;
                }
            };
        }

        buf = [0; 4096];
        n = 0;
    }
}
