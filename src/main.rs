#![no_std]
#![no_main]

use cortex_m_rt::entry;
use panic_halt as _;
use rtt_target::{rprintln, rtt_init_print};
use stm32f1xx_hal::{gpio::*, pac, prelude::*};
use lib_74hc959::ShiftRegister;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let dp = pac::Peripherals::take().unwrap();

    let mut gpio_b = dp.GPIOB.split();
    let lock_pin = gpio_b.pb12.into_push_pull_output(&mut gpio_b.crh).erase();
    let clock_pin = gpio_b.pb13.into_push_pull_output(&mut gpio_b.crh).erase();
    let data_pin = gpio_b.pb14.into_push_pull_output(&mut gpio_b.crh).erase();

    let mut shift_register = ShiftRegister::new(lock_pin, clock_pin, data_pin);

    shift_register.write_array(&[0, 0, 0, 0, 0, 0, 0, 0]);

    loop {}
}
