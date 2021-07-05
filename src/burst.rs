use crate::socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    OUT1,
    OUT2,
}

impl std::convert::From<Source> for String {
    fn from(source: Source) -> Self {
        let s = match source {
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

impl std::convert::From<Mode> for String {
    fn from(mode: Mode) -> Self {
        match mode {
            Mode::CONTINUOUS => "CONTINUOUS",
            Mode::BURST => "BURST",
            Mode::STREAM => "STREAM",
        }.to_owned()
    }
}

impl std::str::FromStr for Mode {
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
    socket: Socket,
}

impl crate::Module for Burst {
    fn new(socket: Socket) -> Self {
        Burst {
            socket,
        }
    }
}

impl Burst {
    /**
     * Set burst (pulse) mode.
     *
     * Red Pitaya will generate R number of N periods of signal and then stop. Time between bursts
     * is P.
     */
    pub fn set_mode(&self, source: Source, mode: Mode) {
        self.socket.send(format!("{}:BURS:STAT {}", Into::<String>::into(source), Into::<String>::into(mode)));
    }

    /**
     * Set burst (pulse) mode.
     */
    pub fn get_mode(&self, source: Source) -> Result<Mode, String> {
        self.socket.send(format!("{}:BURS:STAT?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set N number of periods in one burst.
     */
    pub fn set_count(&self, source: Source, count: u32) {
        self.socket.send(format!("{}:BURS:NCYC {}", Into::<String>::into(source), count));
    }

    /**
     * Get number of periods in one burst.
     */
    pub fn get_count(&self, source: Source) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send(format!("{}:BURS:NCYC?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set R number of repeated bursts.
     */
    pub fn set_repetitions(&self, source: Source, repetitions: u32) {
        self.socket.send(format!("{}:BURS:NOR {}", Into::<String>::into(source), repetitions));
    }

    /**
     * Get number of repeated bursts.
     */
    pub fn get_repetitions(&self, source: Source) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send(format!("{}:BURS:NOR?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }

    /**
     * Set P total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn set_period(&self, source: Source, period: u32) {
        self.socket.send(format!("{}:BURS:INT:PER {}", Into::<String>::into(source), period));
    }

    /**
     * Get total time of one burst in in micro seconds.
     *
     * This includes the signal and delay.
     */
    pub fn get_period(&self, source: Source) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send(format!("{}:BURS:INT:PER?", Into::<String>::into(source)))
            .unwrap()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_mode() {
        let (rx, rp) = crate::test::create_client();

        rp.burst.set_mode(crate::burst::Source::OUT2, crate::burst::Mode::BURST);
        assert_eq!("SOUR2:BURS:STAT BURST\r\n", rx.recv().unwrap());

        assert_eq!(rp.burst.get_mode(crate::burst::Source::OUT2), Ok(crate::burst::Mode::BURST));
    }

    #[test]
    fn test_count() {
        let (rx, rp) = crate::test::create_client();

        rp.burst.set_count(crate::burst::Source::OUT2, 3);
        assert_eq!("SOUR2:BURS:NCYC 3\r\n", rx.recv().unwrap());

        assert_eq!(rp.burst.get_count(crate::burst::Source::OUT2), Ok(3));
    }

    #[test]
    fn test_repetitions() {
        let (rx, rp) = crate::test::create_client();

        rp.burst.set_repetitions(crate::burst::Source::OUT1, 5);
        assert_eq!("SOUR1:BURS:NOR 5\r\n", rx.recv().unwrap());

        assert_eq!(rp.burst.get_repetitions(crate::burst::Source::OUT1), Ok(5));
    }

    #[test]
    fn test_period() {
        let (rx, rp) = crate::test::create_client();

        rp.burst.set_period(crate::burst::Source::OUT2, 1_000_000);
        assert_eq!("SOUR2:BURS:INT:PER 1000000\r\n", rx.recv().unwrap());

        assert_eq!(rp.burst.get_period(crate::burst::Source::OUT2), Ok(1_000_000));
    }
}
