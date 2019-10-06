use crate::socket::Socket;

pub trait Pin : std::convert::Into<String> {
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum OutputPin {
    AOUT0,
    AOUT1,
    AOUT2,
    AOUT3,
}

impl Pin for OutputPin {
}

impl std::convert::Into<String> for OutputPin {
    fn into(self) -> String {
        let s = match self {
            OutputPin::AOUT0 => "AOUT0",
            OutputPin::AOUT1 => "AOUT1",
            OutputPin::AOUT2 => "AOUT2",
            OutputPin::AOUT3 => "AOUT3",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum InputPin {
    AIN0,
    AIN1,
    AIN2,
    AIN3,
}

impl Pin for InputPin {
}

impl std::convert::Into<String> for InputPin {
    fn into(self) -> String {
        let s = match self {
            InputPin::AIN0 => "AIN0",
            InputPin::AIN1 => "AIN1",
            InputPin::AIN2 => "AIN2",
            InputPin::AIN3 => "AIN3",
        };

        String::from(s)
    }
}

#[derive(Clone)]
pub struct Analog {
    socket: Socket,
}

impl crate::Module for Analog {
    fn new(socket: Socket) -> Self {
        Analog {
            socket,
        }
    }
}

impl Analog {
    /**
     * Sets analog outputs to default values (0V).
     */
    pub fn reset(&self) {
        self.socket.send("ANALOG:RST");
    }

    /**
     * Set analog voltage on slow analog outputs.
     *
     * Voltage range of slow analog outputs is: 0 - 1.8 V
     */
    pub fn set_value(&self, pin: OutputPin, value: f32) {
        self.socket.send(format!("ANALOG:PIN {},{}", Into::<String>::into(pin), value));
    }

    /**
     * Read analog voltage from slow analog inputs.
     *
     * Voltage range of slow analog inputs is: 0 3.3 V
     */
    pub fn get_value<P>(&self, pin: P) -> Result<f32, <f32 as std::str::FromStr>::Err>
        where P: Pin
    {
        self.socket.send(format!("ANALOG:PIN? {}", Into::<String>::into(pin)))
            .unwrap()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_reset() {
        let (rx, rp) = crate::test::create_client();

        rp.analog.reset();
        assert_eq!("ANALOG:RST\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_value() {
        let (rx, rp) = crate::test::create_client();

        rp.analog.set_value(crate::analog::OutputPin::AOUT1, 1.34);
        assert_eq!("ANALOG:PIN AOUT1,1.34\r\n", rx.recv().unwrap());

        #[cfg(feature = "mock")]
        assert_eq!(rp.analog.get_value(crate::analog::InputPin::AIN1), Ok(1.34));

        #[cfg(not(feature = "mock"))]
        assert!(rp.analog.get_value(crate::analog::InputPin::AIN1).is_ok());
    }
}
