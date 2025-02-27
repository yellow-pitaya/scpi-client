extern crate redpitaya_scpi;

use redpitaya_scpi::Redpitaya;
use redpitaya_scpi::digital::{Led, State};
use std::thread::sleep;
use std::time::Duration;

fn main() {
    let redpitaya = Redpitaya::new("192.168.1.5:5000".to_owned());

    loop {
        let leds = [
            Led::LED0,
            Led::LED1,
            Led::LED2,
            Led::LED3,
            Led::LED4,
            Led::LED5,
            Led::LED6,
            Led::LED7,
        ];

        for led in leds.iter() {
            redpitaya.digital.set_state(*led, State::HIGH);
            sleep(Duration::from_millis(100));
            redpitaya.digital.set_state(*led, State::LOW);
        }

        for led in leds.iter().rev() {
            redpitaya.digital.set_state(*led, State::HIGH);
            sleep(Duration::from_millis(100));
            redpitaya.digital.set_state(*led, State::LOW);
        }
    }
}
