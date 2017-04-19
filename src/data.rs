use socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    RAW,
    VOLTS,
}

impl ::std::fmt::Display for Unit {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Unit::RAW => "RAW",
            &Unit::VOLTS => "VOLTS",
        };

        write!(f, "{}", display)
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Format {
    FLOAT,
    ASCII,
}

impl ::std::fmt::Display for Format {
    fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
        let display = match self {
            &Format::FLOAT => "FLOAT",
            &Format::ASCII => "ASCII",
        };

        write!(f, "{}", display)
    }
}

#[derive(Clone)]
pub struct Data {
    socket: Socket,
}

impl Data {
    pub fn new(socket: Socket) -> Self {
        Data {
            socket: socket,
        }
    }

    /**
     * Returns current position of write pointer.
     */
    pub fn get_write_pointer(&mut self) -> u32 {
        self.socket.send("ACQ:WPOS?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    /**
     * Returns position where trigger event appeared.
     */
    pub fn get_trigger_position(&mut self) -> u32 {
        self.socket.send("ACQ:TPOS?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    /**
     * Selects units in which acquired data will be returned.
     */
    pub fn set_units(&mut self, unit: Unit) {
        self.socket.send(format!("ACQ:DATA:UNITS {}", unit));
    }

    /**
     * Selects format acquired data will be returned.
     */
    pub fn set_format(&mut self, format: Format) {
        self.socket.send(format!("ACQ:DATA:FORMAT {}", format));
    }

    /**
     * Read samples from start to stop position.
     *
     * start = {0,1,...,16384}
     * stop_pos = {0,1,...116384}
     */
    pub fn read_slice(&mut self, source: ::acquire::Source, start: u16, end: u16) -> String {
        self.socket.send(format!("ACQ:{}:DATA:STA:END? {},{}", source, start, end));

        self.socket.receive()
    }

    /**
     * Read `m` samples from start position on.
     */
    pub fn read(&mut self, source: ::acquire::Source, start: u16, len: u32) -> String {
        self.socket.send(format!("ACQ:{}:DATA:STA:N? {},{}", source, start, len));

        self.socket.receive()
    }

    /**
     * Read full buf.
     *
     * Size starting from oldest sample in buffer (this is first sample after
     * trigger delay). Trigger delay by default is set to zero (in samples or
     * in seconds). If trigger delay is set to zero it will read full buf.
     * Size starting from trigger.
     */
    pub fn read_all(&mut self, source: ::acquire::Source) -> String {
        self.socket.send(format!("ACQ:{}:DATA?", source));

        self.socket.receive()
    }

    /**
     * Read m samples after trigger delay, starting from oldest sample in buffer
     * (this is first sample after trigger delay).
     *
     * Trigger delay by default is set to zero (in samples or in seconds). If
     * trigger delay is set to zero it will read m samples starting from trigger.
     */
    pub fn read_oldest(&mut self, source: ::acquire::Source, len: u32) -> String {
        self.socket.send(format!("ACQ:{}:DATA:OLD:N? {}", source, len));

        self.socket.receive()
    }

    /**
     * Read ``m`` samples before trigger delay.
     *
     * Trigger delay by default is set to zero (in samples or in seconds). If
     * trigger delay is set to zero it will read m samples before trigger.
     */
    pub fn read_latest(&mut self, source: ::acquire::Source, len: u32) -> String {
        self.socket.send(format!("ACQ:{}:DATA:LAT:N? {}", source, len));

        self.socket.receive()
    }

    /**
     * Returns buffer size.
     */
    pub fn buffer_size(&mut self) -> u32 {
        self.socket.send("ACQ:BUF:SIZE?");

        self.socket.receive()
            .parse()
            .unwrap()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_write_pointer() {
        let (_, mut data) = create_data();

        assert_eq!(data.get_write_pointer(), 1024);
    }

    #[test]
    fn test_get_write_pointer_at_trigger() {
        let (_, mut data) = create_data();

        assert_eq!(data.get_trigger_position(), 512);
    }

    #[test]
    fn test_set_units() {
        let (rx, mut data) = create_data();

        data.set_units(::data::Unit::VOLTS);
        assert_eq!("ACQ:DATA:UNITS VOLTS\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_set_format() {
        let (rx, mut data) = create_data();

        data.set_format(::data::Format::FLOAT);
        assert_eq!("ACQ:DATA:FORMAT FLOAT\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_read_slice() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_slice(::acquire::Source::IN1, 10, 13), "{123,231,-231}");
    }

    #[test]
    fn test_read() {
        let (_, mut data) = create_data();

        assert_eq!(data.read(::acquire::Source::IN1, 10, 3), "{1.2,3.2,-1.2}");
    }

    #[test]
    fn test_read_all() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_all(::acquire::Source::IN1), "{1.2,3.2,-1.2}");
    }

    #[test]
    fn test_read_oldest() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_oldest(::acquire::Source::IN1, 2), "{3.2,-1.2}");
    }

    #[test]
    fn test_read_latest() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_latest(::acquire::Source::IN1, 2), "{1.2,3.2}");
    }

    #[test]
    fn test_buffer_size() {
        let (_, mut data) = create_data();

        assert_eq!(data.buffer_size(), 16384);
    }

    fn create_data() -> (::std::sync::mpsc::Receiver<String>, ::data::Data) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::data::Data::new(socket))
    }
}
