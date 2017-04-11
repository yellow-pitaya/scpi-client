use socket::Socket;

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

    pub fn set_direction(&mut self, pin: &str, direction: Direction) {
        self.socket.send(format!("DIG:PIN:DIR {},{}", direction, pin));
    }

    pub fn set_state(&mut self, pin: &str, state: State) {
        self.socket.send(format!("DIG:PIN {},{}", pin, state));
    }

    pub fn get_state(&mut self, pin: &str) -> State {
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

        digital.set_direction("DIO0_N", ::digital::Direction::IN);
        assert_eq!("DIG:PIN:DIR IN,DIO0_N\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_direction_out() {
        let (rx, mut digital) = create_digital();

        digital.set_direction("DIO0_N", ::digital::Direction::OUT);
        assert_eq!("DIG:PIN:DIR OUT,DIO0_N\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_state() {
        let (rx, mut digital) = create_digital();

        digital.set_state("DIO0_N", ::digital::State::LOW);
        assert_eq!("DIG:PIN DIO0_N,0\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_state() {
        let (_, mut digital) = create_digital();

        assert_eq!(digital.get_state("DIO0_N"), ::digital::State::HIGH);
    }

    fn create_digital() -> (::std::sync::mpsc::Receiver<String>, ::digital::Digital) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::digital::Digital::new(socket))
    }
}
