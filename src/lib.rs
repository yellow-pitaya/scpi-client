pub mod acquire;
pub mod analog;
pub mod burst;
pub mod data;
pub mod digital;
pub mod general;
pub mod generator;
pub mod trigger;
pub mod socket;

trait Module {
    fn new(socket: socket::Socket) -> Self;
}

#[derive(Clone)]
pub struct Redpitaya {
    pub acquire: acquire::Acquire,
    pub analog: analog::Analog,
    pub burst: burst::Burst,
    pub data: data::Data,
    pub digital: digital::Digital,
    pub general: general::General,
    pub generator: generator::Generator,
    pub trigger: trigger::Trigger,
}

impl Redpitaya {
    pub fn new(addr: String) -> Self
    {
        let socket = socket::Socket::new(addr);

        Self {
            acquire: acquire::Acquire::new(socket.clone()),
            analog: analog::Analog::new(socket.clone()),
            burst: burst::Burst::new(socket.clone()),
            data: data::Data::new(socket.clone()),
            digital: digital::Digital::new(socket.clone()),
            general: general::General::new(socket.clone()),
            generator: generator::Generator::new(socket.clone()),
            trigger: trigger::Trigger::new(socket.clone()),
        }
    }
}

impl std::default::Default for Redpitaya {
    fn default() -> Self {
        Self::new("127.0.0.1:5000".to_owned())
    }
}

#[cfg(test)]
mod test {
    use std::io::Read;
    use std::io::Write;

    pub fn create_client() -> (std::sync::mpsc::Receiver<String>, crate::Redpitaya) {
        let (addr, rx) = crate::test::launch_server();

        (rx, crate::Redpitaya::new(addr))
    }

    pub fn launch_server() -> (String, std::sync::mpsc::Receiver<String>) {
        let addr = next_test_ip4();
        let listener = std::net::TcpListener::bind(format!("{}", addr))
            .unwrap();

        let (tx, rx) = std::sync::mpsc::channel();

        std::thread::spawn(move || {
            loop {
                if let Ok((mut stream, _)) =  listener.accept() {
                    let tx = tx.clone();

                    std::thread::spawn(move || {
                        handle_client(&mut stream, tx);
                    });
                }
            }
        });

        (addr, rx)
    }

    static PORT: std::sync::atomic::AtomicUsize = std::sync::atomic::ATOMIC_USIZE_INIT;

    fn next_test_ip4() -> String {
        let port = PORT.fetch_add(1, std::sync::atomic::Ordering::SeqCst) as u16 + base_port();

        format!("127.0.0.1:{}", port)
    }

    // The bots run multiple builds at the same time, and these builds
    // all want to use ports. This function figures out which workspace
    // it is running in and assigns a port range based on it.
    fn base_port() -> u16 {
        let cwd = std::env::current_dir()
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

    fn handle_client(stream: &mut std::net::TcpStream, tx: std::sync::mpsc::Sender<String>) {
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
        let socket = crate::socket::Socket::new("192.168.1.5:5000".to_owned());

        socket.send(message.clone())
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
            "ACQ:SOUR1:DATA:STA:END? 10,12" => "{123,231,-231}",
            "ACQ:SOUR1:DATA:STA:N? 10,3" => "{1.2,3.2,-1.2}",
            "ACQ:SOUR1:DATA?" => "{1.2,3.2,-1.2}",
            "ACQ:SOUR1:DATA:OLD:N? 2" => "{3.2,-1.2}",
            "ACQ:SOUR1:DATA:LAT:N? 2" => "{1.2,3.2}",
            "ACQ:BUF:SIZE?" => "16384",
            "ACQ:TRIG:STAT?" => "WAIT",
            "ACQ:TRIG:DLY?" => "2314",
            "ACQ:TRIG:DLY:NS?" => "128ns",
            "ACQ:TRIG:HYST?" => "0.75",
            "ACQ:TRIG:LEV?" => "0.4",
            "ANALOG:PIN? AIN1" => "1.34",
            "DIG:PIN? DIO0_N" => "1",
            "OUTPUT2:STATE?" => "ON",
            "SOUR1:DCYC?" => "1.0",
            "SOUR1:FREQ:FIX?" => "1000",
            "SOUR2:FREQ:FIX?" => "8.82604e+06",
            "SOUR1:FUNC?" => "SINE",
            "SOUR1:PHAS?" => "180",
            "SOUR1:TRAC:DATA:DATA?" => "1,0.5,0.2",
            "SOUR1:TRIG:SOUR?" => "BURST",
            "SOUR1:VOLT?" => "-0.5",
            "SOUR1:VOLT:OFFS?" => "0.3",
            "SOUR2:BURS:STAT?" => "BURST",
            "SOUR2:BURS:NCYC?" => "3",
            "SOUR1:BURS:NOR?" => "5",
            "SOUR2:BURS:INT:PER?" => "1000000",
            _ => return None,
        }.to_owned();

        response.push_str("\r\n");

        Some(response)
    }
}
