
#![no_main]
#![no_std]

use cortex_m_rt::entry;

use core::fmt::Write;
use microbit::{self as _, hal::uarte::{Baudrate, Parity, Uarte}};
use microbit::Board;
use microbit::pac::twim0::frequency::FREQUENCY_A;

use microbit::hal::delay::Delay;

use microbit::hal::Twim;
use lsm303agr::{Lsm303agr, MagMode, MagOutputDataRate};

#[entry]
fn main() -> ! {
    let board = Board::take().unwrap();

    let mut serial = Uarte::new(
        board.UARTE0,
        board.uart.into(),
        Parity::EXCLUDED,
        Baudrate::BAUD115200,
    );

    let i2c = Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100);
    let mut sensor = Lsm303agr::new_with_i2c(i2c);

    let mut delay = Delay::new(board.SYST);

    sensor.init().unwrap();
    sensor.set_mag_mode_and_odr(&mut delay, MagMode::HighResolution, MagOutputDataRate::Hz10).unwrap();

    let Ok(mut sensor) = sensor.into_mag_continuous() else {
        panic!("Panic!");
    };

    loop {
        if sensor.accel_status().unwrap().xyz_new_data() {
            let data = sensor.magnetic_field().unwrap();
            writeln!(serial, "Field: x {} y {} z {}", data.x_nt(), data.y_nt(), data.z_nt()).ok();
        }
    }
}

#[panic_handler]
fn painc_handler(_info: &core::panic::PanicInfo) -> ! {
    /* write!(serial, "panicked: {}", panic_info.message()); */
    loop {}
}
