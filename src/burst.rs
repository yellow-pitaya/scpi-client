use Module;
use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    OUT1,
    OUT2,
}

impl ::std::convert::Into<String> for Source {
    fn into(self) -> String {
        let s = match self {
            Source::OUT1 => "SOUR1",
            Source::OUT2 => "SOUR2",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Mode {
    CONTINUOUS,
    BURST,
    STREAM
}

impl ::std::convert::Into<String> for Mode {
    fn into(self) -> String {
        match self {
            Mode::CONTINUOUS => "CONTINUOUS",
            Mode::BURST => "BURST",
            Mode::STREAM => "STREAM",
        }.to_owned()
    }
}

impl ::std::str::FromStr for Mode {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CONTINUOUS" => Ok(Mode::CONTINUOUS),
            "BURST" => Ok(Mode::BURST),
            "STREAM" => Ok(Mode::STREAM),
            state => Err(format!("Unknow state '{}'", state)),
        }
    }
}

#[derive(Clone)]
pub struct Burst {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for Burst {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl Burst {
    pub fn new(socket: Socket) -> Self {
        Burst {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Set burst (pulse) mode.
     *
     * Red Pitaya will generate R number of N periods of signal and then stop. Time between bursts
     * is P.
     */
    pub fn set_mode(&self, source: Source, mode: Mode) {
        self.send(format!("{}:BURS:STAT {}", Into::<String>::into(source), Into::<String>::into(mode)));
    }

    /**
     * Set burst (pulse) mode.
     */
    pub fn get_mode(&self, source: Source) -> Result<Mode, String> {
        self.send(format!("{}:BURS:STAT?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set N number of periods in one burst.
     */
    pub fn set_count(&self, source: Source, count: u32) {
        self.send(format!("{}:BURS:NCYC {}", Into::<String>::into(source), count));
    }

    /**
     * Get number of periods in one burst.
     */
    pub fn get_count(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:NCYC?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set R number of repeated bursts.
     */
    pub fn set_repetitions(&self, source: Source, repetitions: u32) {
        self.send(format!("{}:BURS:NOR {}", Into::<String>::into(source), repetitions));
    }

    /**
     * Get number of repeated bursts.
     */
    pub fn get_repetitions(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:NOR?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }

    /**
     * Set P total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn set_period(&self, source: Source, period: u32) {
        self.send(format!("{}:BURS:INT:PER {}", Into::<String>::into(source), period));
    }

    /**
     * Get total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn get_period(&self, source: Source) -> Result<u32, <u32 as ::std::str::FromStr>::Err> {
        self.send(format!("{}:BURS:INT:PER?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_mode() {
        let (rx, burst) = create_burst();

        burst.set_mode(::burst::Source::OUT2, ::burst::Mode::BURST);
        assert_eq!("SOUR2:BURS:STAT BURST\r\n", rx.recv().unwrap());

        assert_eq!(burst.get_mode(::burst::Source::OUT2), Ok(::burst::Mode::BURST));
    }

    #[test]
    fn test_count() {
        let (rx, burst) = create_burst();

        burst.set_count(::burst::Source::OUT2, 3);
        assert_eq!("SOUR2:BURS:NCYC 3\r\n", rx.recv().unwrap());

        assert_eq!(burst.get_count(::burst::Source::OUT2), Ok(3));
    }

    #[test]
    fn test_repetitions() {
        let (rx, burst) = create_burst();

        burst.set_repetitions(::burst::Source::OUT1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());

        assert_eq!(burst.get_repetitions(::burst::Source::OUT1), Ok(5));
    }

    #[test]
    fn test_period() {
        let (rx, burst) = create_burst();

        burst.set_period(::burst::Source::OUT2, 1_000_000);
        assert_eq!("SOUR2:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());

        assert_eq!(burst.get_period(::burst::Source::OUT2), Ok(1_000_000));
    }

    fn create_burst() -> (::std::sync::mpsc::Receiver<String>, ::burst::Burst) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::burst::Burst::new(socket))
    }
}
