use socket::Socket;

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

    pub fn set_state(&mut self, pin: &str, state: u8) {
        self.socket.send(format!("DIG:PIN {},{}", pin, state));
    }

    pub fn get_state(&mut self, pin: &str) -> u8 {
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

        digital.set_state("DIO0_N", 1);
        assert_eq!("DIG:PIN DIO0_N,1\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_state() {
        let (_, mut digital) = create_digital();

        assert_eq!(digital.get_state("DIO0_N"), 1);
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
