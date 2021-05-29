#![no_std]
#![no_main]

use arxrobot::prelude::*;
use panic_halt as _;

#[arxrobot::entry]
fn main() -> ! {
    let dp = arxrobot::Peripherals::take().unwrap();
    
    let mut pins = arxrobot::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    let mut led = pins.d13.into_output(&mut pins.ddr);
    
    let mut serial = arxrobot::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        57600.into_baudrate()
    );
    serial.
    
    loop {
        led.toggle();
        ufmt::uwrite!(&mut serial, "Toggling LED!\r").void_unwrap();
        arxrobot::delay_ms(2000);
    }
}