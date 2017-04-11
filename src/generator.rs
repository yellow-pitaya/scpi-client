use socket::Socket;

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

    pub fn start(&mut self, source: usize) {
        self.socket.send(format!("OUTPUT{}:STATE ON", source));
        self.states[source].started = true;
    }

    pub fn stop(&mut self, source: usize) {
        self.socket.send(format!("OUTPUT{}:STATE OFF", source));
        self.states[source].started = false;
    }

    pub fn is_started(&self, source: usize) -> bool {
        self.states[source].started
    }

    pub fn set_form<S>(&mut self, source: usize, form: S) where S: Into<String> {
        self.states[source].form = form.into();
        self.socket.send(format!("SOUR{}:FUNC {}", source, self.states[source].form))
    }

    pub fn get_form(&self, source: usize) -> String {
        self.states[source].form.clone()
    }

    pub fn set_amplitude(&mut self, source: usize, amplitude: f32) {
        self.socket.send(format!("SOUR{}:VOLT {}", source, amplitude));
        self.states[source].amplitude = amplitude;
    }

    pub fn get_amplitude(&self, source: usize) -> f32 {
        self.states[source].amplitude
    }

    pub fn set_offset(&mut self, source: usize, offset: f32) {
        self.socket.send(format!("SOUR{}:VOLT:OFFS {}", source, offset));
    }

    pub fn set_phase(&mut self, source: usize, phase: i32) {
        self.socket.send(format!("SOUR{}:PHAS {}", source, phase));
    }

    pub fn set_dcyc(&mut self, source: usize, dcyc: u32) {
        self.socket.send(format!("SOUR{}:DCYC {}", source, dcyc));
        self.states[source].dcyc = dcyc;
    }

    pub fn get_dcyc(&self, source: usize) -> u32 {
        self.states[source].dcyc
    }

    pub fn arbitrary_waveform(&mut self, source: usize, data: Vec<f32>) {
        let mut data = data.iter()
            .fold(String::new(), |acc, e| {
                format!("{}{},", acc, e)
            });
        data.pop();

        self.socket.send(format!("SOUR{}:TRAC:DATA:DATA {}", source, data));
    }

    pub fn set_frequency(&mut self, source: usize, frequency: u32) {
        self.socket.send(format!("SOUR{}:FREQ:FIX {}", source, frequency));
        self.states[source].frequency = frequency;
    }

    pub fn get_frequency(&self, source: usize) -> u32 {
        self.states[source].frequency
    }

    pub fn set_trigger_source(&mut self, source: usize, trigger: &str) {
        self.socket.send(format!("SOUR{}:TRIG:SOUR {}", source, trigger));
    }

    pub fn trigger(&mut self, source: usize) {
        self.socket.send(format!("SOUR{}:TRIG:IMM", source));
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

        generator.start(1);
        assert_eq!("OUTPUT1:STATE ON\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_stop() {
        let (rx, mut generator) = create_generator();

        generator.stop(1);
        assert_eq!("OUTPUT1:STATE OFF\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_is_started() {
        let (_, mut generator) = create_generator();

        assert_eq!(generator.is_started(1), false);
        generator.start(1);
        assert_eq!(generator.is_started(1), true);
        generator.stop(1);
        assert_eq!(generator.is_started(1), false);
    }

    #[test]
    fn test_form() {
        let (rx, mut generator) = create_generator();

        generator.set_form(1, "SINE");
        assert_eq!("SOUR1:FUNC SINE\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_amplitude() {
        let (rx, mut generator) = create_generator();

        generator.set_amplitude(1, -0.9);
        assert_eq!("SOUR1:VOLT -0.9\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_offset() {
        let (rx, mut generator) = create_generator();

        generator.set_offset(1, -1.0);
        assert_eq!("SOUR1:VOLT:OFFS -1\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_phase() {
        let (rx, mut generator) = create_generator();

        generator.set_phase(1, -360);
        assert_eq!("SOUR1:PHAS -360\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_dcyc() {
        let (rx, mut generator) = create_generator();

        generator.set_dcyc(1, 100);
        assert_eq!("SOUR1:DCYC 100\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_arbitrary_waveform() {
        let (rx, mut generator) = create_generator();

        generator.arbitrary_waveform(1, vec![1.0, 0.5, 0.2]);
        assert_eq!("SOUR1:TRAC:DATA:DATA 1,0.5,0.2\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_frequency() {
        let (rx, mut generator) = create_generator();

        generator.set_frequency(1, 500);
        assert_eq!("SOUR1:FREQ:FIX 500\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_get_frequency() {
        let (_, generator) = create_generator();

        assert_eq!(generator.get_frequency(1), 1000);
    }

    #[test]
    fn test_set_trigger_source() {
        let (rx, mut generator) = create_generator();

        generator.set_trigger_source(1, "EXT");
        assert_eq!("SOUR1:TRIG:SOUR EXT\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_trigger() {
        let (rx, mut generator) = create_generator();

        generator.trigger(1);
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
