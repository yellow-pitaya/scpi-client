use socket::Socket;

pub trait Pin: ::std::fmt::Display {
}

pub enum Gpio {
    DIO0_N,
    DIO0_P,
    DIO1_N,
    DIO1_P,
    DIO2_N,
    DIO2_P,
    DIO3_N,
    DIO3_P,
    DIO4_N,
    DIO4_P,
    DIO5_N,
    DIO5_P,
    DIO6_N,
    DIO6_P,
    DIO7_N,
    DIO7_P,
}

impl Pin for Gpio {
}

impl ::std::fmt::Display for Gpio {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Gpio::DIO0_N => "DIO0_N",
            &Gpio::DIO0_P => "DIO0_P",
            &Gpio::DIO1_N => "DIO1_N",
            &Gpio::DIO1_P => "DIO1_P",
            &Gpio::DIO2_N => "DIO2_N",
            &Gpio::DIO2_P => "DIO2_P",
            &Gpio::DIO3_N => "DIO3_N",
            &Gpio::DIO3_P => "DIO3_P",
            &Gpio::DIO4_N => "DIO4_N",
            &Gpio::DIO4_P => "DIO4_P",
            &Gpio::DIO5_N => "DIO5_N",
            &Gpio::DIO5_P => "DIO5_P",
            &Gpio::DIO6_N => "DIO6_N",
            &Gpio::DIO6_P => "DIO6_P",
            &Gpio::DIO7_N => "DIO7_N",
            &Gpio::DIO7_P => "DIO7_P",
        };

        write!(f, "{}", display)
    }
}

pub enum Led {
    LED0,
    LED1,
    LED2,
    LED3,
    LED4,
    LED5,
    LED6,
    LED7,
    LED8,
}

impl Pin for Led {
}

impl ::std::fmt::Display for Led {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Led::LED0 => "LED0",
            &Led::LED1 => "LED1",
            &Led::LED2 => "LED2",
            &Led::LED3 => "LED3",
            &Led::LED4 => "LED4",
            &Led::LED5 => "LED5",
            &Led::LED6 => "LED6",
            &Led::LED7 => "LED7",
            &Led::LED8 => "LED8",
        };

        write!(f, "{}", display)
    }
}

#[derive(Debug, PartialEq)]
pub enum State {
    LOW,
    HIGH,
}

impl ::std::fmt::Display for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &State::LOW => "0",
            &State::HIGH => "1",
        };

        write!(f, "{}", display)
    }
}

impl ::std::str::FromStr for State {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(State::LOW),
            "1" => Ok(State::HIGH),
            _ => Err(()),
        }
    }
}

pub enum Direction {
    OUT,
    IN,
}

impl ::std::fmt::Display for Direction {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Direction::OUT => "OUT",
            &Direction::IN => "IN",
        };

        write!(f, "{}", display)
    }
}

pub struct Digital {
    socket: Socket,
}

impl Digital {
    pub fn new(socket: Socket) -> Self {
        Digital {
            socket: socket,
        }
    }

    /**
     * Set direction of digital pins to output or input.
     */
    pub fn set_direction<P>(&mut self, pin: P, direction: Direction) where P: Pin {
        self.socket.send(format!("DIG:PIN:DIR {},{}", direction, pin));
    }

    /**
     * Set state of digital outputs to 1 (HIGH) or 0 (LOW).
     */
    pub fn set_state<P>(&mut self, pin: P, state: State) where P: Pin {
        self.socket.send(format!("DIG:PIN {},{}", pin, state));
    }

    /**
     * Get state of digital inputs and outputs.
     */
    pub fn get_state<P>(&mut self, pin: P) -> State where P: Pin {
        self.socket.send(format!("DIG:PIN? {}", pin));

        self.socket.receive()
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_set_direction_in() {
        let (rx, mut digital) = create_digital();

        digital.set_direction(::digital::Gpio::DIO0_N, ::digital::Direction::IN);
        assert_eq!("DIG:PIN:DIR IN,DIO0_N\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_direction_out() {
        let (rx, mut digital) = create_digital();

        digital.set_direction(::digital::Led::LED0, ::digital::Direction::OUT);
        assert_eq!("DIG:PIN:DIR OUT,LED0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_state() {
        let (rx, mut digital) = create_digital();

        digital.set_state(::digital::Gpio::DIO0_N, ::digital::State::LOW);
        assert_eq!("DIG:PIN DIO0_N,0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_state() {
        let (_, mut digital) = create_digital();

        assert_eq!(digital.get_state(::digital::Gpio::DIO0_N), ::digital::State::HIGH);
    }

    fn create_digital() -> (::std::sync::mpsc::Receiver<String>, ::digital::Digital) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::digital::Digital::new(socket))
    }
}
