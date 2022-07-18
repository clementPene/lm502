//! Driver crate for the LM502 LoraWAN transceiver
//!
//!
//! 
//! These examples uses a NUCLEO STM32F103RB
//!
//! This driver is built on top of [`embedded-hal`], which means it is portable
//! and can be used on any platform that implements the `embedded-hal` API.
//! 
//! Be careful to use 3.3V output pin to connect to the module. 
//!
//! [`embedded-hal`]: https://crates.io/crates/embedded-hal
#![no_main]
#![no_std]
#![deny(missing_docs)]

use embedded_hal::{
    serial::{Read, Write},
};

/// Entry point
pub struct LM502<SERIAL> 
where
    SERIAL: Read<u8> + Write<u8>,
{
	serial:         SERIAL,
}

/// list all available commands 
pub enum at_command { /// set baudrate
    CGBR, /// Set baudrate
    CJOINMODE, /// Set Join Method
    CDEVEUI, /// Set DevEUI
    CAPPEUI, /// Set AppEUI
    CAPPKEY, /// Set AppKey
    CDEVADDR, /// Set DevAddr
    CAPPSKEY, ///Set AppSKey
    CNWKSKEY, ///Set NwkSKey
    CFREQBANDMASK, /// Set Channel Mask
    CULDLMODE, /// Set uplink / downlink same frequency or different frequency
    CWORKMODE, /// Set working mode
    CCLASS, /// Set class
    CBL, /// Check battery level
    CSTATUS, /// enquiry module status
    CJOIN, /// Join network
    DTRX, /// Transmit and Receive data
    DRX, /// Receive Data
    CCONFIRM, /// Set uplink transmit type
    CAPPPORT, /// Set uplink port
    CDATARATE, /// Set data rate
    CRSSI, /// Enquiry signal strength
    CNBTRIALS, /// Set uplink retires
    CRM, /// Set uplink mode
    CTXP, /// Set Transmit Power
    CLINKCHECK, /// Check network connection
    CADRL, /// Enable / Disable ADR
    CRXP, /// Set RX window parameter
    CRX1DELAY, /// Set RX1 window delay
    CSAVE, /// Save MAC parameters
    CRESTORE, /// Restore MAC parameters
    CPINGSLOTINFOREQ, /// PinSlotInfo request
    CADDMUTICAST, /// Add multiply broadcast address
    CDELMUTICAST, /// Delete multiply broadcast numbers
    CNUMMUTICAST, /// Enquiry multiply broadcast numbers
    IREBOOT, /// Reboot module
    ILOGLVL, // Set log level
}


