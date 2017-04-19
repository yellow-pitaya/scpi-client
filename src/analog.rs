use socket::Socket;

pub trait Pin: ::std::fmt::Display {
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

impl ::std::fmt::Display for OutputPin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &OutputPin::AOUT0 => "AOUT0",
            &OutputPin::AOUT1 => "AOUT1",
            &OutputPin::AOUT2 => "AOUT2",
            &OutputPin::AOUT3 => "AOUT3",
        };

        write!(f, "{}", display)
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

impl ::std::fmt::Display for InputPin {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &InputPin::AIN0 => "AIN0",
            &InputPin::AIN1 => "AIN1",
            &InputPin::AIN2 => "AIN2",
            &InputPin::AIN3 => "AIN3",
        };

        write!(f, "{}", display)
    }
}

#[derive(Clone)]
pub struct Analog {
    socket: Socket,
}

impl Analog {
    pub fn new(socket: Socket) -> Self {
        Analog {
            socket: socket,
        }
    }

    /**
     * Set analog voltage on slow analog outputs.
     *
     * Voltage range of slow analog outputs is: 0 - 1.8 V
     */
    pub fn set_value(&mut self, pin: OutputPin, value: f32) {
        self.socket.send(format!("ANALOG:PIN {},{}", pin, value));
    }

    /**
     * Read analog voltage from slow analog inputs.
     *
     * Voltage range of slow analog inputs is: 0 3.3 V
     */
    pub fn get_value<P>(&mut self, pin: P) -> f32
        where P: Pin
    {
        self.socket.send(format!("ANALOG:PIN? {}", pin));

        self.socket.receive()
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set_value() {
        let (rx, mut analog) = create_analog();

        analog.set_value(::analog::OutputPin::AOUT2, 1.34);
        assert_eq!("ANALOG:PIN AOUT2,1.34\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_value() {
        let (_, mut analog) = create_analog();

        assert_eq!(analog.get_value(::analog::InputPin::AIN1), 1.34);
    }

    fn create_analog() -> (::std::sync::mpsc::Receiver<String>, ::analog::Analog) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::analog::Analog::new(socket))
    }
}
