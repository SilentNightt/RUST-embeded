#![no_main]
#![no_std]

use cortex_m_rt::entry;

use core::fmt::Write;
use microbit::{self as _, hal::uarte::{Baudrate, Parity, Uarte}};
use microbit::Board;

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    writeln!(serial, "Hello from microbit!");

    loop {}
}

#[panic_handler]
fn painc_handler(_info: &core::panic::PanicInfo) -> ! {
    /* write!(serial, "panicked: {}", panic_info.message()); */
    loop {}
}