#![deny(unsafe_code)]
#![no_main]
#![no_std]

use panic_itm;

use cortex_m_rt::entry;

use nrf52832_hal::prelude::*;
use cortex_m::{asm::bkpt, iprint, iprintln, peripheral::ITM};

#[entry]
fn main() -> ! {
    let mut itm = init();

    iprintln!(&mut itm.stim[0], "Hello, world!");

    loop {}
}


fn init() -> ITM {
    let p = cortex_m::Peripherals::take().unwrap();

    p.ITM
}
