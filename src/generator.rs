use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TriggerSource {
    EXT_PE,
    EXT_NE,
    INT,
    GATED,
}

impl ::std::fmt::Display for TriggerSource {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &TriggerSource::EXT_PE => "EXT_PE",
            &TriggerSource::EXT_NE => "EXT_NE",
            &TriggerSource::INT => "INT",
            &TriggerSource::GATED => "GATED",
        };

        write!(f, "{}", display)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Form {
    SINE,
    SQUARE,
    TRIANGLE,
    SAWU,
    SAWD,
    PWM,
    ARBITRARY,
    UNKNOW,
}

impl ::std::fmt::Display for Form {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Form::SINE => "SINE",
            &Form::SQUARE => "SQUARE",
            &Form::TRIANGLE => "TRIANGLE",
            &Form::SAWU => "SAWU",
            &Form::SAWD => "SAWD",
            &Form::PWM => "PWM",
            &Form::ARBITRARY => "ARBITRARY",
            &Form::UNKNOW => "UNKNOW",
        };

        write!(f, "{}", display)
    }
}

impl ::std::convert::From<String> for Form {
    fn from(s: String) -> Self {
        match s.as_str() {
            "SINE" => Form::SINE,
            "SQUARE" => Form::SQUARE,
            "TRIANGLE" => Form::TRIANGLE,
            "SAWU" => Form::SAWU,
            "SAWD" => Form::SAWD,
            "PWM" => Form::PWM,
            "ARBITRARY" => Form::ARBITRARY,
            form => {
                warn!("Unknow signal form {}", form);
                Form::UNKNOW
            },
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
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

// @TODO impl std::slice::SliceIndex<generator::State> instead
impl ::std::convert::Into<usize> for Source {
    fn into(self) -> usize {
        match self {
            Source::OUT1 => 0,
            Source::OUT2 => 1,
        }
    }
}

#[derive(Clone)]
pub struct Generator {
    socket: ::std::cell::RefCell<Socket>,
}

impl Generator {
    pub fn new(socket: Socket) -> Self {
        Generator {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Enable fast analog outputs.
     */
    pub fn start(&self, source: Source) {
        self.set_state(&source, "ON");
    }

    /**
     * Disable fast analog outputs.
     */
    pub fn stop(&self, source: Source) {
        self.set_state(&source, "OFF");
    }

    fn set_state(&self, source: &Source, state: &str) {
        let output = match *source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.send(format!("{}:STATE {}", output, state));
    }

    pub fn is_started(&self, source: Source) -> bool {
        self.send(format!("{}:STATE?", source));

        self.receive()
            .parse::<u8>()
            .unwrap() == 1
    }

    /**
     * Set frequency of fast analog outputs.
     */
    pub fn set_frequency(&self, source: Source, frequency: u32) {
        self.send(format!("{}:FREQ:FIX {}", source, frequency));
    }

    /**
     * Get frequency of fast analog outputs.
     */
    pub fn get_frequency(&self, source: Source) -> u32 {
        self.send(format!("{}:FREQ:FIX?", source));

        self.receive()
            .parse()
            .unwrap()
    }

    /**
     * Set waveform of fast analog outputs.
     */
    pub fn set_form(&self, source: Source, form: Form) {
        self.send(format!("{}:FUNC {}", source, form));
    }

    pub fn get_form(&self, source: Source) -> Form {
        self.send(format!("{}:FUNC?", source));

        self.receive()
            .into()
    }

    /**
     * Set amplitude voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_amplitude(&self, source: Source, amplitude: f32) {
        self.send(format!("{}:VOLT {}", source, amplitude));
    }

    /**
     * Get amplitude voltage of fast analog outputs.
     */
    pub fn get_amplitude(&self, source: Source) -> f32 {
        self.send(format!("{}:VOLT?", source));

        self.receive()
            .parse()
            .unwrap()
    }

    /**
     * Set offset voltage of fast analog outputs.
     *
     * Amplitude + offset value must be less than maximum output range ± 1V
     */
    pub fn set_offset(&self, source: Source, offset: f32) {
        self.send(format!("{}:VOLT:OFFS {}", source, offset));
    }

    /**
     * Get offset voltage of fast analog outputs.
     */
    pub fn get_offset(&self, source: Source) -> f32 {
        self.send(format!("{}:VOLT:OFFS?", source));

        self.receive()
            .parse()
            .unwrap()
    }

    /**
     * Set phase of fast analog outputs.
     */
    pub fn set_phase(&self, source: Source, phase: i32) {
        self.send(format!("{}:PHAS {}", source, phase));
    }

    /**
     * Set duty cycle of PWM waveform.
     */
    pub fn set_duty_cycle(&self, source: Source, dcyc: f32) {
        self.send(format!("{}:DCYC {}", source, dcyc));
    }

    /**
     * Get duty cycle of PWM waveform.
     */
    pub fn get_duty_cycle(&self, source: Source) -> f32 {
        self.send(format!("{}:DCYC?", source));

        self.receive()
            .parse()
            .unwrap()
    }

    /**
     * Import data for arbitrary waveform generation.
     */
    pub fn arbitrary_waveform(&self, source: Source, data: Vec<f32>) {
        let mut data = data.iter()
            .fold(String::new(), |acc, e| {
                format!("{}{},", acc, e)
            });
        data.pop();

        self.send(format!("{}:TRAC:DATA:DATA {}", source, data));
    }

    /**
     * Set trigger source for selected signal.
     */
    pub fn set_trigger_source(&self, source: Source, trigger: TriggerSource) {
        self.send(format!("{}:TRIG:SOUR {}", source, trigger));
    }

    /**
     * Triggers selected source immediately.
     */
    pub fn trigger(&self, source: Source) {
        self.send(format!("{}:TRIG:IMM", source));
    }

    /**
     * Triggers both sources immediately.
     */
    pub fn trigger_all(&self) {
        self.send("TRIG:IMM");
    }

    /**
     * Reset generator to default settings.
     */
    pub fn reset(&self) {
        self.send("GEN:RST");
    }

    fn send<D>(&self, message: D)
        where D: ::std::fmt::Display
    {
        let mut socket = self.socket.borrow_mut();

        socket.send(message);
    }

    fn receive(&self) -> String {
        let mut socket = self.socket.borrow_mut();

        socket.receive()
    }
}

#[cfg(test)]
mod test {
    macro_rules! generator_assert {
        ($f:ident, $e:expr) => {
            let (rx, generator) = create_generator();

            generator.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_start() {
        let (rx, generator) = create_generator();

        generator.start(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_stop() {
        let (rx, generator) = create_generator();

        generator.stop(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_is_started() {
        let (_, generator) = create_generator();

        assert_eq!(generator.is_started(::generator::Source::OUT1), true);
    }

    #[test]
    fn test_set_frequency() {
        let (rx, generator) = create_generator();

        generator.set_frequency(::generator::Source::OUT1, 500);
        assert_eq!("SOUR1:FREQ:FIX 500\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_frequency() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_frequency(::generator::Source::OUT1), 1000);
    }

    #[test]
    fn test_set_form() {
        let (rx, generator) = create_generator();

        generator.set_form(::generator::Source::OUT1, ::generator::Form::SINE);
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_form() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_form(::generator::Source::OUT1), ::generator::Form::SINE);
    }

    #[test]
    fn test_set_amplitude() {
        let (rx, generator) = create_generator();

        generator.set_amplitude(::generator::Source::OUT1, -0.9);
        assert_eq!("SOUR1:VOLT -0.9\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_amplitude() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_amplitude(::generator::Source::OUT1), -1.1);
    }

    #[test]
    fn test_set_offset() {
        let (rx, generator) = create_generator();

        generator.set_offset(::generator::Source::OUT1, -1.0);
        assert_eq!("SOUR1:VOLT:OFFS -1\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_offset() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_offset(::generator::Source::OUT1), 1.2);
    }

    #[test]
    fn test_set_phase() {
        let (rx, generator) = create_generator();

        generator.set_phase(::generator::Source::OUT1, -360);
        assert_eq!("SOUR1:PHAS -360\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_duty_cycle() {
        let (rx, generator) = create_generator();

        generator.set_duty_cycle(::generator::Source::OUT1, 0.5);
        assert_eq!("SOUR1:DCYC 0.5\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_duty_cycle() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_duty_cycle(::generator::Source::OUT1), 1.0);
    }

    #[test]
    fn test_arbitrary_waveform() {
        let (rx, generator) = create_generator();

        generator.arbitrary_waveform(::generator::Source::OUT1, vec![1.0, 0.5, 0.2]);
        assert_eq!("SOUR1:TRAC:DATA:DATA 1,0.5,0.2\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_trigger_source() {
        let (rx, generator) = create_generator();

        generator.set_trigger_source(::generator::Source::OUT1, ::generator::TriggerSource::EXT_PE);
        assert_eq!("SOUR1:TRIG:SOUR EXT_PE\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger() {
        let (rx, generator) = create_generator();

        generator.trigger(::generator::Source::OUT1);
        assert_eq!("SOUR1:TRIG:IMM\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger_all() {
        generator_assert!(trigger_all, "TRIG:IMM\r\n");
    }

    #[test]
    fn test_reset() {
        generator_assert!(reset, "GEN:RST\r\n");
    }

    fn create_generator() -> (::std::sync::mpsc::Receiver<String>, ::generator::Generator) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::generator::Generator::new(socket))
    }
}
