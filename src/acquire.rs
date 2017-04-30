use Module;
use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Gain {
    LV,
    HV,
}

impl ::std::convert::Into<String> for Gain {
    fn into(self) -> String {
        let s = match self {
            Gain::LV => "LV",
            Gain::HV => "HV",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for Gain {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "LV" => Ok(Gain::LV),
            "HV" => Ok(Gain::HV),
            gain => Err(format!("Unknow gain '{}'", gain)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Source {
    IN1,
    IN2,
}

impl ::std::convert::Into<String> for Source {
    fn into(self) -> String {
        let s = match self {
            Source::IN1 => "SOUR1",
            Source::IN2 => "SOUR2",
        };

        String::from(s)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Decimation {
    DEC_1,
    DEC_8,
    DEC_64,
    DEC_1024,
    DEC_8192,
    DEC_65536,
}

impl ::std::convert::Into<String> for Decimation {
    fn into(self) -> String {
        let s = match self {
            Decimation::DEC_1 => "1",
            Decimation::DEC_8 => "8",
            Decimation::DEC_64 => "64",
            Decimation::DEC_1024 => "1024",
            Decimation::DEC_8192 => "8192",
            Decimation::DEC_65536 => "65536",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for Decimation {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "1" => Ok(Decimation::DEC_1),
            "8" => Ok(Decimation::DEC_8),
            "64" => Ok(Decimation::DEC_64),
            "1024" => Ok(Decimation::DEC_1024),
            "8192" => Ok(Decimation::DEC_8192),
            "65536" => Ok(Decimation::DEC_65536),
            decimation => Err(format!("Unknow decimation '{}'", decimation)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum SamplingRate {
    RATE_125MHz,
    RATE_15_6MHz,
    RATE_1_9MHz,
    RATE_103_8kHz,
    RATE_15_2kHz,
    RATE_1_9kHz,
}

impl SamplingRate {
    pub fn get_buffer_duration(self) -> ::std::time::Duration {
        let (s, ns) = match self {
            SamplingRate::RATE_125MHz => (0, 131_072),
            SamplingRate::RATE_15_6MHz => (0, 1_049_000),
            SamplingRate::RATE_1_9MHz => (0, 8_389_000),
            SamplingRate::RATE_103_8kHz => (0, 134_218_000),
            SamplingRate::RATE_15_2kHz => (1, 740_000_000),
            SamplingRate::RATE_1_9kHz => (8, 590_000_000),
        };

        ::std::time::Duration::new(s, ns)
    }
}

impl ::std::fmt::Display for SamplingRate {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &SamplingRate::RATE_125MHz => "125 MHz",
            &SamplingRate::RATE_15_6MHz => "15.6 MHz",
            &SamplingRate::RATE_1_9MHz => "1.9 MHz",
            &SamplingRate::RATE_103_8kHz => "103.8 kHz",
            &SamplingRate::RATE_15_2kHz => "15.2 kHz",
            &SamplingRate::RATE_1_9kHz => "1.9 kHz",
        };

        write!(f, "{}", display)
    }
}

impl ::std::convert::Into<String> for SamplingRate {
    fn into(self) -> String {
        let s = match self {
            SamplingRate::RATE_125MHz => "125MHz",
            SamplingRate::RATE_15_6MHz => "15_6MHz",
            SamplingRate::RATE_1_9MHz => "1_9MHz",
            SamplingRate::RATE_103_8kHz => "103_8kHz",
            SamplingRate::RATE_15_2kHz => "15_2kHz",
            SamplingRate::RATE_1_9kHz => "1_9kHz",
        };

        String::from(s)
    }
}

impl ::std::str::FromStr for SamplingRate {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "125MHz" => Ok(SamplingRate::RATE_125MHz),
            "15_6MHz" => Ok(SamplingRate::RATE_15_6MHz),
            "1_9MHz" => Ok(SamplingRate::RATE_1_9MHz),
            "103_8kHz" => Ok(SamplingRate::RATE_103_8kHz),
            "15_2kHz" => Ok(SamplingRate::RATE_15_2kHz),
            "1_9kHz" => Ok(SamplingRate::RATE_1_9kHz),
            rate => Err(format!("Unknow sampling rate {}", rate)),
        }
    }
}

#[derive(Clone)]
pub struct Acquire {
    socket: ::std::cell::RefCell<Socket>,
    started: bool,
}

impl ::Module for Acquire {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl Acquire {
    pub fn new(socket: Socket) -> Self {
        Acquire {
            socket: ::std::cell::RefCell::new(socket),
            started: false,
        }
    }

    /**
     * Starts acquisition.
     */
    pub fn start(&mut self) {
        self.send("ACQ:START");
        self.started = true;
    }

    /**
     * Stops acquisition.
     */
    pub fn stop(&mut self) {
        self.send("ACQ:STOP");
        self.started = false;
    }

    pub fn is_started(&self) -> bool {
        self.started
    }

    /**
     * Stops acquisition and sets all parameters to default values.
     */
    pub fn reset(&self) {
        self.send("ACQ:RST");
    }

    /**
     * Set decimation factor.
     */
    pub fn set_decimation(&self, decimation: Decimation) {
        self.send(format!("ACQ:DEC {}", Into::<String>::into(decimation)));
    }

    /**
     * Get decimation factor.
     */
    pub fn get_decimation(&self) -> Result<Decimation, String> {
        self.send("ACQ:DEC?");

        self.receive()
            .parse()
    }

    /**
     * Set sampling rate.
     */
    pub fn set_sampling_rate(&self, rate: SamplingRate) {
        self.send(format!("ACQ:SRAT {}", Into::<String>::into(rate)));
    }

    /**
     * Get sampling rate.
     */
    pub fn get_sampling_rate(&self) -> Result<SamplingRate, String> {
        self.send("ACQ:SRAT?");

        self.receive()
            .parse()
    }

    /**
     * Get sampling rate in Hertz.
     */
    pub fn get_sampling_rate_in_hertz(&self) -> Result<u64, <u64 as ::std::str::FromStr>::Err> {
        self.send("ACQ:SRA:HZ?");

        self.receive()
            .replace(" Hz", "")
            .parse()
    }

    /**
     * Enable averaging.
     */
    pub fn enable_average(&self) {
        self.send("ACQ:AVG ON");
    }

    /**
     * Disable averaging.
     */
    pub fn disable_average(&self) {
        self.send("ACQ:AVG OFF");
    }

    /**
     * Get averaging status.
     */
    pub fn is_average_enabled(&self) -> bool {
        self.send("ACQ:AVG?");

        let message = self.receive();

        match message.as_str() {
            "ON" => true,
            _ => false,
        }
    }

    /**
     * Set gain settings to HIGH or LOW.
     *
     * This gain is referring to jumper settings on Red Pitaya fast analog inputs.
     */
    pub fn set_gain(&self, source: Source, gain: Gain) {
        self.send(format!("ACQ:{}:GAIN {}", Into::<String>::into(source), Into::<String>::into(gain)));
    }

    /**
     * Get gain settings to HIGH or LOW.
     */
    pub fn get_gain(&self, source: Source) -> Result<Gain, String> {
        self.send(format!("ACQ:{}:GAIN?", Into::<String>::into(source)));

        self.receive()
            .parse()
    }
}

#[cfg(test)]
mod test {
    macro_rules! acquire_assert {
        ($f:ident, $e:expr) => {
            let (rx, acquire) = create_acquire();

            acquire.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_sampling_rate_get_buffer_duration() {
        let duration = ::std::time::Duration::new(8, 590_000_000);

        assert_eq!(duration, ::acquire::SamplingRate::RATE_1_9kHz.get_buffer_duration());
    }

    #[test]
    fn test_start() {
        let (rx, mut acquire) = create_acquire();

        acquire.start();
        assert_eq!("ACQ:START\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_stop() {
        let (rx, mut acquire) = create_acquire();

        acquire.stop();
        assert_eq!("ACQ:STOP\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_is_started() {
        let (_, mut acquire) = create_acquire();

        assert_eq!(acquire.is_started(), false);
        acquire.start();
        assert_eq!(acquire.is_started(), true);
        acquire.stop();
        assert_eq!(acquire.is_started(), false);
    }

    #[test]
    fn test_reset() {
        acquire_assert!(reset, "ACQ:RST\r\n");
    }

    #[test]
    fn test_set_decimation() {
        let (rx, acquire) = create_acquire();

        acquire.set_decimation(::acquire::Decimation::DEC_8);
        assert_eq!("ACQ:DEC 8\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_decimation() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_decimation(), Ok(::acquire::Decimation::DEC_1));
    }

    #[test]
    fn test_set_sampling_rate() {
        let (rx, acquire) = create_acquire();

        acquire.set_sampling_rate(::acquire::SamplingRate::RATE_125MHz);
        assert_eq!("ACQ:SRAT 125MHz\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_sampling_rate() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_sampling_rate(), Ok(::acquire::SamplingRate::RATE_1_9kHz));
    }

    #[test]
    fn test_get_sampling_rate_in_hertz() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_sampling_rate_in_hertz(), Ok(125_000_000));
    }

    #[test]
    fn test_enable_average() {
        acquire_assert!(enable_average, "ACQ:AVG ON\r\n");
    }

    #[test]
    fn test_disable_average() {
        acquire_assert!(disable_average, "ACQ:AVG OFF\r\n");
    }

    #[test]
    fn test_is_average_enabled() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.is_average_enabled(), true);
    }

    #[test]
    fn test_set_gain() {
        let (rx, acquire) = create_acquire();

        acquire.set_gain(::acquire::Source::IN1, ::acquire::Gain::LV);
        assert_eq!("ACQ:SOUR1:GAIN LV\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_gain() {
        let (_, acquire) = create_acquire();

        assert_eq!(acquire.get_gain(::acquire::Source::IN1), Ok(::acquire::Gain::HV));
    }

    fn create_acquire() -> (::std::sync::mpsc::Receiver<String>, ::acquire::Acquire) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::acquire::Acquire::new(socket))
    }
}
