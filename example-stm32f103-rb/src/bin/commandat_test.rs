#![no_main]
#![no_std]

use example_stm32f103_rb as _; // global logger + panicking-behavior + memory layout

use at_commands::builder::CommandBuilder;


#[cortex_m_rt::entry]
fn main() -> ! {
    defmt::println!("Hello, world!");

    let mut buffer = [0; 128];

    // Make a query command
    let result = CommandBuilder::create_query(&mut buffer, true)
        .named("+MYQUERY")
        .finish()
        .unwrap();

    // Buffer now contains "AT+MYQUERY?"
    // Copy or DMA the resulting slice to the device.

    // Make a set command
    let result = CommandBuilder::create_set(&mut buffer, false)
        .named("+MYSET")
        .with_int_parameter(42)
        .finish()
        .unwrap();

    // Buffer now contains "+MYSET=42"
    // Copy or DMA the resulting slice to the device.


    loop {


    }

}

