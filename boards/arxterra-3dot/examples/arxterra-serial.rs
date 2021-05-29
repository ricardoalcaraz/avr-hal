#![no_std]
#![no_main]

extern crate panic_halt;
use arxrobot::prelude::*;

#[arxrobot::entry]
fn main() -> ! {
    let dp = arxrobot::Peripherals::take().unwrap();

    let mut pins = arxrobot::Pins::new(dp.PORTB, dp.PORTC, dp.PORTD, dp.PORTE, dp.PORTF);
    let mut led_pin = pins.d13.into_output(&mut pins.ddr);

    let mut serial = arxrobot::Serial::new(
        dp.USART1,
        pins.d0,
        pins.d1.into_output(&mut pins.ddr),
        9600.into_baudrate(),
    );

    ufmt::uwriteln!(&mut serial, "Hello from Arduino!\r").void_unwrap();

    loop {
        led_pin.toggle().void_unwrap();
        // Read a byte from the serial connection
        arxrobot::delay_ms(1000);
        // Answer
        nb::block!(ufmt::uwriteln!(&mut serial, "This is a test!\n").void_unwrap());
    }
}

pub mod usart {
    pub use avr_hal_generic::usart::*;

    /// Serial interface based on ATmega32U4's USART1 peripheral
    pub type Usart<CLOCK, IMODE> = Usart<
        portd::PD2<crate::port::mode::Input<IMODE>>,
        portd::PD3<crate::port::mode::Output>,
        CLOCK,
    >;
}
