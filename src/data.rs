use crate::socket::Socket;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Unit {
    RAW,
    VOLTS,
}

impl std::convert::Into<String> for Unit {
    fn into(self) -> String {
        let s = match self {
            Unit::RAW => "RAW",
            Unit::VOLTS => "VOLTS",
        };

        String::from(s)
    }
}

impl std::str::FromStr for Unit  {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "RAW" => Ok(Unit::RAW),
            "VOLTS" => Ok(Unit::VOLTS),
            unit => Err(format!("Unknow unit '{}'", unit)),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Format {
    ASCII,
    BIN,
}

impl std::convert::Into<String> for Format {
    fn into(self) -> String {
        let s = match self {
            Format::ASCII => "ASCII",
            Format::BIN => "BIN",
        };

        String::from(s)
    }
}

#[derive(Clone)]
pub struct Data {
    socket: Socket,
}

impl crate::Module for Data {
    fn new(socket: Socket) -> Self {
        Data {
            socket,
        }
    }
}

impl Data {
    /**
     * Returns current position of write pointer.
     */
    pub fn get_write_pointer(&self) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send("ACQ:WPOS?")
            .unwrap()
            .parse()
    }

    /**
     * Returns position where trigger event appeared.
     */
    pub fn get_trigger_position(&self) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send("ACQ:TPOS?")
            .unwrap()
            .parse()
    }

    /**
     * Selects units in which acquired data will be returned.
     */
    pub fn set_units(&self, unit: Unit) {
        self.socket.send(format!("ACQ:DATA:UNITS {}", Into::<String>::into(unit)));
    }

    /**
     * Get units in which acquired data will be returned.
     */
    pub fn get_units(&self) -> Result<Unit, String> {
        self.socket.send("ACQ:DATA:UNITS?")
            .unwrap()
            .parse()
    }

    /**
     * Selects format acquired data will be returned.
     */
    pub fn set_format(&self, format: Format) {
        self.socket.send(format!("ACQ:DATA:FORMAT {}", Into::<String>::into(format)));
    }

    /**
     * Read samples from start to stop position.
     *
     * start = {0,1,...,16384}
     * stop_pos = {0,1,...16384}
     */
    pub fn read_slice(&self, source: crate::acquire::Source, start: u16, end: u16) -> Vec<f64> {
        let data = self.socket.send(format!("ACQ:{}:DATA:STA:END? {},{}", Into::<String>::into(source), start, end))
            .unwrap();

        self.parse(data)
    }

    /**
     * Read `m` samples from start position on.
     */
    pub fn read(&self, source: crate::acquire::Source, start: u16, len: u32) -> Vec<f64> {
        let data = self.socket.send(format!("ACQ:{}:DATA:STA:N? {},{}", Into::<String>::into(source), start, len))
            .unwrap();

        self.parse(data)
    }

    /**
     * Read full buf.
     *
     * Size starting from oldest sample in buffer (this is first sample after
     * trigger delay). Trigger delay by default is set to zero (in samples or
     * in seconds). If trigger delay is set to zero it will read full buf.
     * Size starting from trigger.
     */
    pub fn read_all(&self, source: crate::acquire::Source) -> Vec<f64> {
        let data = self.socket.send(format!("ACQ:{}:DATA?", Into::<String>::into(source)))
            .unwrap();

        self.parse(data)
    }

    fn parse(&self, data: String) -> Vec<f64> {
        data.trim_matches(|c: char| c == '{' || c == '}' || c == '!' || c.is_alphabetic())
            .split(',')
            .map(|s| {
                match s.parse::<f64>() {
                    Ok(f) => f,
                    Err(_) => {
                        log::error!("Invalid data '{}'", s);
                        0.0
                    },
                }
            }).collect()
    }

    /**
     * Read m samples after trigger delay, starting from oldest sample in buffer
     * (this is first sample after trigger delay).
     *
     * Trigger delay by default is set to zero (in samples or in seconds). If
     * trigger delay is set to zero it will read m samples starting from trigger.
     */
    pub fn read_oldest(&self, source: crate::acquire::Source, len: u32) -> Vec<f64> {
        let data = self.socket.send(format!("ACQ:{}:DATA:OLD:N? {}", Into::<String>::into(source), len))
            .unwrap();

        self.parse(data)
    }

    /**
     * Read ``m`` samples before trigger delay.
     *
     * Trigger delay by default is set to zero (in samples or in seconds). If
     * trigger delay is set to zero it will read m samples before trigger.
     */
    pub fn read_latest(&self, source: crate::acquire::Source, len: u32) -> Vec<f64> {
        let data = self.socket.send(format!("ACQ:{}:DATA:LAT:N? {}", Into::<String>::into(source), len))
            .unwrap();

        self.parse(data)
    }

    /**
     * Returns buffer size.
     */
    pub fn buffer_size(&self) -> Result<u32, <u32 as std::str::FromStr>::Err> {
        self.socket.send("ACQ:BUF:SIZE?")
            .unwrap()
            .parse()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn test_get_write_pointer() {
        let (_, rp) = crate::test::create_client();

        #[cfg(feature = "mock")]
        assert_eq!(rp.data.get_write_pointer(), Ok(1024));

        #[cfg(not(feature = "mock"))]
        assert!(rp.data.get_write_pointer().is_ok());
    }

    #[test]
    fn test_get_write_pointer_at_trigger() {
        let (_, rp) = crate::test::create_client();

        #[cfg(feature = "mock")]
        assert_eq!(rp.data.get_trigger_position(), Ok(512));

        #[cfg(not(feature = "mock"))]
        assert!(rp.data.get_trigger_position().is_ok());
    }

    #[test]
    fn test_units() {
        let (rx, rp) = crate::test::create_client();

        rp.data.set_units(crate::data::Unit::RAW);
        assert_eq!("ACQ:DATA:UNITS RAW\r\n", rx.recv().unwrap());

        assert_eq!(rp.data.get_units(), Ok(crate::data::Unit::RAW));
    }

    #[test]
    fn test_set_format() {
        let (rx, rp) = crate::test::create_client();

        rp.data.set_format(crate::data::Format::BIN);
        assert_eq!("ACQ:DATA:FORMAT BIN\r\n", rx.recv().unwrap());

        rp.data.set_format(crate::data::Format::ASCII);
    }

    #[test]
    fn test_read_slice() {
        let (_, rp) = crate::test::create_client();

        let vec = rp.data.read_slice(crate::acquire::Source::IN1, 10, 12);

        #[cfg(feature = "mock")]
        assert_eq!(vec, vec![123.0, 231.0, -231.0]);

        #[cfg(not(feature = "mock"))]
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_read() {
        let (_, rp) = crate::test::create_client();

        let vec = rp.data.read(crate::acquire::Source::IN1, 10, 3);

        #[cfg(feature = "mock")]
        assert_eq!(vec, vec![1.2, 3.2, -1.2]);

        #[cfg(not(feature = "mock"))]
        assert_eq!(vec.len(), 3);
    }

    #[test]
    fn test_read_all() {
        let (_, rp) = crate::test::create_client();

        let vec = rp.data.read_all(crate::acquire::Source::IN1);

        #[cfg(feature = "mock")]
        assert_eq!(vec, vec![1.2, 3.2, -1.2]);

        #[cfg(not(feature = "mock"))]
        assert!(vec.len() > 0);
    }

    #[test]
    fn test_read_oldest() {
        let (_, rp) = crate::test::create_client();

        let vec = rp.data.read_oldest(crate::acquire::Source::IN1, 2);

        #[cfg(feature = "mock")]
        assert_eq!(vec, vec![3.2, -1.2]);

        #[cfg(not(feature = "mock"))]
        assert!(vec.len() > 0);
    }

    #[test]
    fn test_read_latest() {
        let (_, rp) = crate::test::create_client();

        let vec = rp.data.read_latest(crate::acquire::Source::IN1, 2);

        #[cfg(feature = "mock")]
        assert_eq!(vec, vec![1.2, 3.2]);

        #[cfg(not(feature = "mock"))]
        assert!(vec.len() > 0);
    }

    #[test]
    fn test_buffer_size() {
        let (_, rp) = crate::test::create_client();

        assert_eq!(rp.data.buffer_size(), Ok(16384));
    }
}
