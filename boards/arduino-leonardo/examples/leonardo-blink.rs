#![no_std]
#![no_main]

use arduino_leonardo::prelude::*;
use panic_halt as _;

#[arduino_leonardo::entry]
fn main() -> ! {
    let dp = arduino_leonardo::Peripherals::take().unwrap();

    let mut pins = arduino_leonardo::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    let mut led2 = pins.d13;
    led2.set_high().void_unwrap();

    loop {
        led2.toggle();
        arduino_leonardo::delay_ms(200);
    }
}
