use socket::Socket;

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

pub struct Data {
    socket: Socket,
}

impl Data {
    pub fn new(socket: Socket) -> Self {
        Data {
            socket: socket,
        }
    }

    pub fn get_write_pointer(&mut self) -> u32 {
        self.socket.send("ACQ:WPOS?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    pub fn get_trigger_position(&mut self) -> u32 {
        self.socket.send("ACQ:TPOS?");

        self.socket.receive()
            .parse()
            .unwrap()
    }

    pub fn set_units(&mut self, unit: Unit) {
        self.socket.send(format!("ACQ:DATA:UNITS {}", unit));
    }

    pub fn set_format(&mut self, format: Format) {
        self.socket.send(format!("ACQ:DATA:FORMAT {}", format));
    }

    pub fn read_slice(&mut self, source: u8, start: u32, end: u32) -> String {
        self.socket.send(format!("ACQ:SOUR{}:DATA:STA:END? {},{}", source, start, end));

        self.socket.receive()
    }

    pub fn read(&mut self, source: u8, start: u32, len: u32) -> String {
        self.socket.send(format!("ACQ:SOUR{}:DATA:STA:N? {},{}", source, start, len));

        self.socket.receive()
    }

    pub fn read_all(&mut self, source: u8) -> String {
        self.socket.send(format!("ACQ:SOUR{}:DATA?", source));

        self.socket.receive()
    }

    pub fn read_oldest(&mut self, source: u8, len: u32) -> String {
        self.socket.send(format!("ACQ:SOUR{}:DATA:OLD:N? {}", source, len));

        self.socket.receive()
    }

    pub fn read_latest(&mut self, source: u8, len: u32) -> String {
        self.socket.send(format!("ACQ:SOUR{}:DATA:LAT:N? {}", source, len));

        self.socket.receive()
    }

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

        assert_eq!(data.read_slice(1, 10, 13), "{123,231,-231}");
    }

    #[test]
    fn test_read() {
        let (_, mut data) = create_data();

        assert_eq!(data.read(1, 10, 3), "{1.2,3.2,-1.2}");
    }

    #[test]
    fn test_read_all() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_all(1), "{1.2,3.2,-1.2}");
    }

    #[test]
    fn test_read_oldest() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_oldest(1, 2), "{3.2,-1.2}");
    }

    #[test]
    fn test_read_latest() {
        let (_, mut data) = create_data();

        assert_eq!(data.read_latest(1, 2), "{1.2,3.2}");
    }

    #[test]
    fn test_buffer_size() {
        let (_, mut data) = create_data();

        assert_eq!(data.buffer_size(), 16384);
    }

    fn create_data() -> (::std::sync::mpsc::Receiver<String>, ::data::Data) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(
            format!("{}", addr.ip()).as_str(),
            addr.port()
        );

        (rx, ::data::Data::new(socket))
    }
}
