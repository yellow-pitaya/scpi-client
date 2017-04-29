use Module;
use socket::Socket;

#[derive(Clone)]
pub struct General {
    socket: ::std::cell::RefCell<Socket>,
}

impl ::Module for General {
    fn get_socket<'a>(&'a self) -> ::std::cell::RefMut<'a, ::socket::Socket> {
        self.socket.borrow_mut()
    }
}

impl General {
    pub fn new(socket: Socket) -> Self {
        General {
            socket: ::std::cell::RefCell::new(socket),
        }
    }

    /**
     * Initializes the library. It must be called first, before any other
     * library method.
     */
    pub fn init(&self) {
        self.send("RP:INit");
    }

    /**
     * Resets all modules.
     */
    pub fn reset(&self) {
        self.send("RP:REset");
    }

    /**
     * Releases the library resources. It must be called last, after library is
     * not used anymore. Typically before application exits.
     */
    pub fn release(&self) {
        self.send("RP:RELease");
    }

    /**
     * Load specified bitstream version (0.93 or 0.94) in the fpga.
     */
    pub fn fpga_load_bitstream(&self, version: f32) {
        self.send(format!("RP:FPGABITREAM {}", version));
    }

    /**
     * Enable digital loop.
     *
     * This internally connect output to input.
     */
    pub fn enable_digital_loop(&self) {
        self.send("RP:DIG:LOop");
    }
}

#[cfg(test)]
mod test {
    macro_rules! general_assert {
        ($f:ident, $e:expr) => {
            let (rx, general) = create_general();

            general.$f();
            assert_eq!($e, rx.recv().unwrap());
        }
    }

    #[test]
    fn test_init() {
        general_assert!(init, "RP:INit\r\n");
    }

    #[test]
    fn test_reset() {
        general_assert!(reset, "RP:REset\r\n");
    }

    #[test]
    fn test_release() {
        general_assert!(release, "RP:RELease\r\n");
    }

    #[test]
    fn test_fpga_load_bitstream() {
        let (rx, general) = create_general();

        general.fpga_load_bitstream(0.93);
        assert_eq!("RP:FPGABITREAM 0.93\r\n", rx.recv().unwrap());
    }

    #[test]
    fn test_enable_digital_loop() {
        general_assert!(enable_digital_loop, "RP:DIG:LOop\r\n");
    }

    fn create_general() -> (::std::sync::mpsc::Receiver<String>, ::general::General) {
        let (addr, rx) = ::test::launch_server();
        let socket = ::socket::Socket::new(addr);

        (rx, ::general::General::new(socket))
    }
}
