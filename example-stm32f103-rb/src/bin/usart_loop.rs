//! Serial interface loopback test
//!
//! You have to short the TX and RX pins to make this program work

//#![allow(clippy::empty_loop)]
//#![deny(unsafe_code)]
#![no_main]
#![no_std]

use example_stm32f103_rb as _; // global logger + panicking-behavior + memory layout

use nb::block;

use stm32f1xx_hal::{
    pac,
    prelude::*,
    serial::{Config, Serial},
};
// use unwrap_infallible::UnwrapInfallible;

#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    // prelude: create handles to the peripherals and registers
    let p = pac::Peripherals::take().unwrap();
    // let cp = cortex_m::Peripherals::take().unwrap();
    let mut flash = p.FLASH.constrain();
    let mut _rcc = p.RCC.constrain();
    let clocks = _rcc.cfgr.freeze(&mut flash.acr);
    let mut afio = p.AFIO.constrain();
    let mut gpioa = p.GPIOA.split();

    // USART1 on Pins A9 and A10
    let pin_tx = gpioa.pa9.into_alternate_push_pull(&mut gpioa.crh);
    let pin_rx = gpioa.pa10;
    // Create an interface struct for USART1 with 9600 Baud
    let serial = Serial::usart1(
        p.USART1,
        (pin_tx, pin_rx),
        &mut afio.mapr,
        Config::default()
            .baudrate(9_600.bps())
            .wordlength_9bits()
            .parity_none(),
        clocks,
    );

    // Switching the 'Word' type parameter for the 'Read' and 'Write' traits between u8 and u16.
    let serial = serial.with_u16_data();
    let serial = serial.with_u8_data();

    // Separate into tx and rx channels
    let (tx, rx) = serial.split();

    // Switch tx to u16.
    let mut tx = tx.with_u16_data();


    // Switch tx back to u8
    // let mut tx = tx.with_u8_data();

    // Write 'R' to the USART
    // block!(tx.write(b'R')).ok();

    // Switch rx to u16.
    let mut rx = rx.with_u16_data();

    // Switch rx back to u8.
    // let mut rx = rx.with_u8_data();

    // Receive a data from the USART and store it in "received"
    // let received: u8 = block!(rx.read()).unwrap();
    // defmt::println!("recieved : {:?}", received);

    loop {
        // Write data to the USART.
        // Depending on the configuration, only the lower 7, 8, or 9 bits are used.
        block!(tx.write(0x1FF)).ok();

        // Receive a data from the USART and store it in "received"
        let received: u16 = block!(rx.read()).unwrap();
        defmt::println!("recieved : {:?}", received);

    }
}