#![no_std]
#![no_main]

use arxrobot::prelude::*;
use panic_halt as _;
extern crate rmp;

#[arxrobot::entry]
fn main() -> ! {
    let dp = arxrobot::Peripherals::take().unwrap();

    let mut pins = arxrobot::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);

    let mut led0 = pins.led_rx.into_output(&mut pins.ddr);
    let mut led1 = pins.led_tx.into_output(&mut pins.ddr);
    let mut led2 = pins.d13.into_output(&mut pins.ddr);

    led0.set_high().void_unwrap();
    led1.set_high().void_unwrap();
    led2.set_high().void_unwrap();

    let mut leds = [led0.downgrade(), led1.downgrade(), led2.downgrade()];

    loop {
        for i in 0..3 {
            leds[i].toggle().void_unwrap();
            leds[(i + 2) % 3].toggle().void_unwrap();
            arxrobot::delay_ms(200);
        }
    }
}
