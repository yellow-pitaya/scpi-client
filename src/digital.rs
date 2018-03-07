use socket::Socket;

pub trait Pin : ::std::convert::Into<String> {
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl ::std::convert::Into<String> for Gpio {
    fn into(self) -> String {
        let s = match self {
            Gpio::DIO0_N => "DIO0_N",
            Gpio::DIO0_P => "DIO0_P",
            Gpio::DIO1_N => "DIO1_N",
            Gpio::DIO1_P => "DIO1_P",
            Gpio::DIO2_N => "DIO2_N",
            Gpio::DIO2_P => "DIO2_P",
            Gpio::DIO3_N => "DIO3_N",
            Gpio::DIO3_P => "DIO3_P",
            Gpio::DIO4_N => "DIO4_N",
            Gpio::DIO4_P => "DIO4_P",
            Gpio::DIO5_N => "DIO5_N",
            Gpio::DIO5_P => "DIO5_P",
            Gpio::DIO6_N => "DIO6_N",
            Gpio::DIO6_P => "DIO6_P",
            Gpio::DIO7_N => "DIO7_N",
            Gpio::DIO7_P => "DIO7_P",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

impl ::std::convert::Into<String> for Led {
    fn into(self) -> String {
        let s = match self {
            Led::LED0 => "LED0",
            Led::LED1 => "LED1",
            Led::LED2 => "LED2",
            Led::LED3 => "LED3",
            Led::LED4 => "LED4",
            Led::LED5 => "LED5",
            Led::LED6 => "LED6",
            Led::LED7 => "LED7",
            Led::LED8 => "LED8",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum State {
    LOW,
    HIGH,
}

impl ::std::convert::Into<String> for State {
    fn into(self) -> String {
        let s = match self {
            State::LOW => "0",
            State::HIGH => "1",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for State {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "0" => Ok(State::LOW),
            "1" => Ok(State::HIGH),
            state => Err(format!("Unknow state '{}'", state)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Direction {
    OUT,
    IN,
}

impl ::std::convert::Into<String> for Direction {
    fn into(self) -> String {
        let s = match self {
            Direction::OUT => "OUT",
            Direction::IN => "IN",
        };

        String::from(s)
    }
}

#[derive(Clone)]
pub struct Digital {
    socket: Socket,
}

impl ::Module for Digital {
    fn new(socket: Socket) -> Self {
        Digital {
            socket,
        }
    }
}

impl Digital {
    /**
     * Sets digital pins to default values.
     *
     * Pins DIO1_P - DIO7_P, RP_DIO0_N - RP_DIO7_N are set al OUTPUT and to LOW.
     * LEDs are set to LOW/OFF
     */
    pub fn reset(&self) {
        self.socket.send("DIG:RST");
    }

    /**
     * Set direction of digital pins to output or input.
     */
    pub fn set_direction<P>(&self, pin: P, direction: Direction)
        where P: Pin
    {
        self.socket.send(format!("DIG:PIN:DIR {},{}", Into::<String>::into(direction), Into::<String>::into(pin)));
    }

    /**
     * Set state of digital outputs to 1 (HIGH) or 0 (LOW).
     */
    pub fn set_state<P>(&self, pin: P, state: State)
        where P: Pin
    {
        self.socket.send(format!("DIG:PIN {},{}", Into::<String>::into(pin), Into::<String>::into(state)));
    }

    /**
     * Get state of digital inputs and outputs.
     */
    pub fn get_state<P>(&self, pin: P) -> Result<State, <State as ::std::str::FromStr>::Err>
        where P: Pin
    {
        self.socket.send(format!("DIG:PIN? {}", Into::<String>::into(pin)))
            .unwrap()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_reset() {
        let (rx, rp) = ::test::create_client();

        rp.digital.reset();
        assert_eq!("DIG:RST\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_direction_in() {
        let (rx, rp) = ::test::create_client();

        rp.digital.set_direction(::digital::Gpio::DIO0_N, ::digital::Direction::IN);
        assert_eq!("DIG:PIN:DIR IN,DIO0_N\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_direction_out() {
        let (rx, rp) = ::test::create_client();

        rp.digital.set_direction(::digital::Led::LED0, ::digital::Direction::OUT);
        assert_eq!("DIG:PIN:DIR OUT,LED0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_state() {
        let (rx, rp) = ::test::create_client();

        rp.digital.set_direction(::digital::Gpio::DIO0_N, ::digital::Direction::OUT);
        assert_eq!("DIG:PIN:DIR OUT,DIO0_N\r\n", rx.recv().unwrap());

        rp.digital.set_state(::digital::Gpio::DIO0_N, ::digital::State::HIGH);
        assert_eq!("DIG:PIN DIO0_N,1\r\n", rx.recv().unwrap());

        assert_eq!(rp.digital.get_state(::digital::Gpio::DIO0_N), Ok(::digital::State::HIGH));
    }
}
