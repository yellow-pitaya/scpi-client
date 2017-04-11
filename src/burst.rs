use socket::Socket;

pub enum Source {
    OUT1,
    OUT2,
}

impl ::std::fmt::Display for Source {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Source::OUT1 => "SOUR1",
            &Source::OUT2 => "SOUR2",
        };

        write!(f, "{}", display)
    }
}

enum State {
    ON,
    OFF,
}

impl ::std::fmt::Display for State {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &State::ON => "ON",
            &State::OFF => "OFF",
        };

        write!(f, "{}", display)
    }
}

pub struct Burst {
    socket: Socket,
}

impl Burst {
    pub fn new(socket: Socket) -> Self {
        Burst {
            socket: socket,
        }
    }

    pub fn enable(&mut self, source: Source) {
        self.socket.send(format!("{}:BURS:STAT {}", source, State::ON));
    }

    pub fn disable(&mut self, source: Source) {
        self.socket.send(format!("{}:BURS:STAT {}", source, State::OFF));
    }

    pub fn set_count(&mut self, source: Source, count: u32) {
        self.socket.send(format!("{}:BURS:NCYC {}", source, count));
    }

    pub fn set_repetitions(&mut self, source: Source, repetitions: u32) {
        self.socket.send(format!("{}:BURS:NOR {}", source, repetitions));
    }

    pub fn set_period(&mut self, source: Source, period: u32) {
        self.socket.send(format!("{}:BURS:INT:PER {}", source, period));
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_enable() {
        let (rx, mut burst) = create_burst();

        burst.enable(::burst::Source::OUT1);
        assert_eq!("SOUR1:BURS:STAT ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_disable() {
        let (rx, mut burst) = create_burst();

        burst.disable(::burst::Source::OUT1);
        assert_eq!("SOUR1:BURS:STAT OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_count() {
        let (rx, mut burst) = create_burst();

        burst.set_count(::burst::Source::OUT1, 3);
        assert_eq!("SOUR1:BURS:NCYC 3\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_repetitions() {
        let (rx, mut burst) = create_burst();

        burst.set_repetitions(::burst::Source::OUT1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_period() {
        let (rx, mut burst) = create_burst();

        burst.set_period(::burst::Source::OUT2, 1_000_000);
        assert_eq!("SOUR2:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());
    }

    fn create_burst() -> (::std::sync::mpsc::Receiver<String>, ::burst::Burst) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::burst::Burst::new(socket))
    }
}
