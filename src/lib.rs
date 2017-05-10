#[macro_use]
extern crate log;

pub mod acquire;
pub mod analog;
pub mod burst;
pub mod data;
pub mod digital;
pub mod general;
pub mod generator;
pub mod socket;
pub mod trigger;

trait Module {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket>;

    fn send<D>(&self, message: D)
        where D: ::std::fmt::Display
    {
        let mut socket = self.get_socket();

        socket.send(message);
    }

    fn receive(&self) -> String {
        let mut socket = self.get_socket();

        socket.receive()
    }
}

#[derive(Clone)]
pub struct Redpitaya {
    pub acquire: acquire::Acquire,
    pub analog: analog::Analog,
    pub burst: burst::Burst,
    pub digital: digital::Digital,
    pub general: general::General,
    pub generator: generator::Generator,
    pub trigger: trigger::Trigger,
    pub data: data::Data,
}

impl Redpitaya {
    pub fn new<S>(addr: S) -> Redpitaya
        where S: ::std::net::ToSocketAddrs
    {
        let socket = socket::Socket::new(addr);

        Redpitaya {
            acquire: acquire::Acquire::new(socket.clone()),
            analog: analog::Analog::new(socket.clone()),
            burst: burst::Burst::new(socket.clone()),
            digital: digital::Digital::new(socket.clone()),
            general: general::General::new(socket.clone()),
            generator: generator::Generator::new(socket.clone()),
            trigger: trigger::Trigger::new(socket.clone()),
            data: data::Data::new(socket.clone()),
        }
    }
}

impl ::std::default::Default for Redpitaya {
    fn default() -> Self {
        Redpitaya::new("127.0.0.1:5000")
    }
}

#[cfg(test)]
mod test {
    use ::std::io::Read;
    use ::std::io::Write;

    pub fn launch_server() -> (::std::net::SocketAddr, ::std::sync::mpsc::Receiver<String>) {
        let addr = next_test_ip4();
        let listener = ::std::net::TcpListener::bind(format!("{}", addr))
            .unwrap();

        let (tx, rx) = ::std::sync::mpsc::channel();

        ::std::thread::spawn(move || {
            loop {
                if let Ok((mut stream, _)) =  listener.accept() {
                    let tx = tx.clone();

                    ::std::thread::spawn(move || {
                        handle_client(&mut stream, tx);
                    });
                }
            }
        });

        (addr, rx)
    }

    static PORT: ::std::sync::atomic::AtomicUsize = ::std::sync::atomic::ATOMIC_USIZE_INIT;

    fn next_test_ip4() -> ::std::net::SocketAddr {
        let port = PORT.fetch_add(1, ::std::sync::atomic::Ordering::SeqCst) as u16 + base_port();
        ::std::net::SocketAddr::V4(::std::net::SocketAddrV4::new(::std::net::Ipv4Addr::new(127, 0, 0, 1), port))
    }

    // The bots run multiple builds at the same time, and these builds
    // all want to use ports. This function figures out which workspace
    // it is running in and assigns a port range based on it.
    fn base_port() -> u16 {
        let cwd = ::std::env::current_dir()
            .unwrap();
        let dirs = [
            "32-opt",
            "32-nopt",
            "musl-64-opt",
            "cross-opt",
            "64-opt",
            "64-nopt",
            "64-opt-vg",
            "64-debug-opt",
            "all-opt",
            "snap3",
            "dist",
        ];

        dirs.iter()
            .enumerate()
            .find(|&(_, dir)| cwd.to_str().unwrap().contains(dir))
            .map(|p| p.0)
            .unwrap_or(0) as u16 * 1000 + 19600
    }

    fn handle_client(stream: &mut ::std::net::TcpStream, tx: ::std::sync::mpsc::Sender<String>) {
        let mut message = String::new();

        loop {
            let mut buffer = [0; 1];

            stream.read(&mut buffer[..])
                .unwrap();
            message.push(buffer[0] as char);

            if buffer[0] == ('\n' as u8) {
                match handle_message(message.clone()) {
                    Some(mut response) => {
                        response.push_str("\r\n");
                        stream.write(response.as_bytes()).unwrap();
                    }
                    None => {
                        tx.send(message).unwrap();
                    }
                };

                message = String::new();
            }
        }
    }

    #[cfg(not(feature = "mock"))]
    fn handle_message(message: String) -> Option<String> {
        let mut socket = ::socket::Socket::new("192.168.1.5:5000");

        socket.send(message.clone());

        if message.contains("?") {
            Some(socket.receive())
        }
        else {
            None
        }
    }

    #[cfg(feature = "mock")]
    fn handle_message(message: String) -> Option<String> {
        let mut response = match message.replace("\r\n", "").as_str() {
            "ACQ:DEC?" => "1",
            "ACQ:AVG?" => "ON",
            "ACQ:DATA:UNITS?" => "RAW",
            "ACQ:SOUR1:GAIN?" => "HV",
            "ACQ:SRAT?" => "125000000 Hz",
            "ACQ:WPOS?" => "1024",
            "ACQ:TPOS?" => "512",
            "ACQ:SOUR1:DATA:STA:END? 10,13" => "{123,231,-231}",
            "ACQ:SOUR1:DATA:STA:N? 10,3" => "{1.2,3.2,-1.2}",
            "ACQ:SOUR1:DATA?" => "{1.2,3.2,-1.2}",
            "ACQ:SOUR1:DATA:OLD:N? 2" => "{3.2,-1.2}",
            "ACQ:SOUR1:DATA:LAT:N? 2" => "{1.2,3.2}",
            "ACQ:BUF:SIZE?" => "16384",
            "ACQ:TRIG:STAT?" => "WAIT",
            "ACQ:TRIG:DLY?" => "2314",
            "ACQ:TRIG:DLY:NS?" => "128ns",
            "ACQ:TRIG:HYST?" => "0.75",
            "ACQ:TRIG:LEV?" => "123mV",
            "ANALOG:PIN? AIN1" => "1.34",
            "DIG:PIN? DIO0_N" => "1",
            "SOUR1:DCYC?" => "1.0",
            "SOUR1:FREQ:FIX?" => "1000",
            "SOUR2:FREQ:FIX?" => "8.82604e+06",
            "SOUR1:FUNC?" => "SINE",
            "SOUR1:PHAS?" => "-180",
            "SOUR1:STATE?" => "1",
            "SOUR1:TRAC:DATA:DATA?" => "1,0.5,0.2",
            "SOUR1:TRIG:SOUR?" => "EXT_NE",
            "SOUR1:VOLT?" => "-1.1",
            "SOUR1:VOLT:OFFS?" => "1.2",
            "SOUR2:BURS:STAT?" => "OFF",
            "SOUR2:BURS:NCYC?" => "3",
            "SOUR1:BURS:NOR?" => "5",
            "SOUR2:BURS:INT:PER?" => "1000000",
            _ => return None,
        }.to_owned();

        response.push_str("\r\n");

        Some(response)
    }
}
