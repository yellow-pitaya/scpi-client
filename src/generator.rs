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

// @TODO impl std::slice::SliceIndex<generator::State> instead
impl ::std::convert::Into<usize> for Source {
    fn into(self) -> usize {
        match self {
            Source::OUT1 => 0,
            Source::OUT2 => 1,
        }
    }
}

struct State {
    started: bool,
    form: String,
    amplitude: f32,
    frequency: u32,
    dcyc: u32,
}

impl State {
    pub fn new() -> Self {
        State {
            started: false,
            form: "SYN".into(),
            amplitude: 1.0,
            frequency: 1000,
            dcyc: 50,
        }
    }
}

pub struct Generator {
    socket: Socket,
    states: [State; 2],
}

impl Generator {
    pub fn new(socket: Socket) -> Self {
        Generator {
            socket: socket,
            states: [
                State::new(),
                State::new(),
            ],
        }
    }

    pub fn start(&mut self, source: Source) {
        self.set_state(&source, "ON");
        self.states[source as usize].started = true;
    }

    pub fn stop(&mut self, source: Source) {
        self.set_state(&source, "OFF");
        self.states[source as usize].started = false;
    }

    fn set_state(&mut self, source: &Source, state: &str) {
        let output = match *source {
            Source::OUT1 => "OUTPUT1",
            Source::OUT2 => "OUTPUT2",
        };

        self.socket.send(format!("{}:STATE {}", output, state));
    }

    pub fn is_started(&self, source: Source) -> bool {
        self.states[source as usize].started
    }

    pub fn set_form<S>(&mut self, source: Source, form: S) where S: Into<String> {
        let form = form.into();

        self.socket.send(format!("{}:FUNC {}", source, form));
        self.states[source as usize].form = form;
    }

    pub fn get_form(&self, source: Source) -> String {
        self.states[source as usize].form.clone()
    }

    pub fn set_amplitude(&mut self, source: Source, amplitude: f32) {
        self.socket.send(format!("{}:VOLT {}", source, amplitude));
        self.states[source as usize].amplitude = amplitude;
    }

    pub fn get_amplitude(&self, source: Source) -> f32 {
        self.states[source as usize].amplitude
    }

    pub fn set_offset(&mut self, source: Source, offset: f32) {
        self.socket.send(format!("{}:VOLT:OFFS {}", source, offset));
    }

    pub fn set_phase(&mut self, source: Source, phase: i32) {
        self.socket.send(format!("{}:PHAS {}", source, phase));
    }

    pub fn set_dcyc(&mut self, source: Source, dcyc: u32) {
        self.socket.send(format!("{}:DCYC {}", source, dcyc));
        self.states[source as usize].dcyc = dcyc;
    }

    pub fn get_dcyc(&self, source: Source) -> u32 {
        self.states[source as usize].dcyc
    }

    pub fn arbitrary_waveform(&mut self, source: Source, data: Vec<f32>) {
        let mut data = data.iter()
            .fold(String::new(), |acc, e| {
                format!("{}{},", acc, e)
            });
        data.pop();

        self.socket.send(format!("{}:TRAC:DATA:DATA {}", source, data));
    }

    pub fn set_frequency(&mut self, source: Source, frequency: u32) {
        self.socket.send(format!("{}:FREQ:FIX {}", source, frequency));
        self.states[source as usize].frequency = frequency;
    }

    pub fn get_frequency(&self, source: Source) -> u32 {
        self.states[source as usize].frequency
    }

    pub fn set_trigger_source(&mut self, source: Source, trigger: &str) {
        self.socket.send(format!("{}:TRIG:SOUR {}", source, trigger));
    }

    pub fn trigger(&mut self, source: Source) {
        self.socket.send(format!("{}:TRIG:IMM", source));
    }

    pub fn trigger_all(&mut self) {
        self.socket.send("TRIG:IMM");
    }

    pub fn reset(&mut self) {
        self.socket.send("GEN:RST");
    }
}

#[cfg(test)]
mod test {
    macro_rules! generator_assert {
        ($f:ident, $e:expr) => {
            let (rx, mut generator) = create_generator();

            generator.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_start() {
        let (rx, mut generator) = create_generator();

        generator.start(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_stop() {
        let (rx, mut generator) = create_generator();

        generator.stop(::generator::Source::OUT2);
        assert_eq!("OUTPUT2:STATE OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_is_started() {
        let (_, mut generator) = create_generator();

        assert_eq!(generator.is_started(::generator::Source::OUT1), false);
        generator.start(::generator::Source::OUT1);
        assert_eq!(generator.is_started(::generator::Source::OUT1), true);
        generator.stop(::generator::Source::OUT1);
        assert_eq!(generator.is_started(::generator::Source::OUT1), false);
    }

    #[test]
    fn test_form() {
        let (rx, mut generator) = create_generator();

        generator.set_form(::generator::Source::OUT1, "SINE");
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_amplitude() {
        let (rx, mut generator) = create_generator();

        generator.set_amplitude(::generator::Source::OUT1, -0.9);
        assert_eq!("SOUR1:VOLT -0.9\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_offset() {
        let (rx, mut generator) = create_generator();

        generator.set_offset(::generator::Source::OUT1, -1.0);
        assert_eq!("SOUR1:VOLT:OFFS -1\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_phase() {
        let (rx, mut generator) = create_generator();

        generator.set_phase(::generator::Source::OUT1, -360);
        assert_eq!("SOUR1:PHAS -360\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_dcyc() {
        let (rx, mut generator) = create_generator();

        generator.set_dcyc(::generator::Source::OUT1, 100);
        assert_eq!("SOUR1:DCYC 100\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_arbitrary_waveform() {
        let (rx, mut generator) = create_generator();

        generator.arbitrary_waveform(::generator::Source::OUT1, vec![1.0, 0.5, 0.2]);
        assert_eq!("SOUR1:TRAC:DATA:DATA 1,0.5,0.2\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_frequency() {
        let (rx, mut generator) = create_generator();

        generator.set_frequency(::generator::Source::OUT1, 500);
        assert_eq!("SOUR1:FREQ:FIX 500\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_frequency() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_frequency(::generator::Source::OUT1), 1000);
    }

    #[test]
    fn test_set_trigger_source() {
        let (rx, mut generator) = create_generator();

        generator.set_trigger_source(::generator::Source::OUT1, "EXT");
        assert_eq!("SOUR1:TRIG:SOUR EXT\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger() {
        let (rx, mut generator) = create_generator();

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
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::generator::Generator::new(socket))
    }
}
