use uart_16550::SerialPort;
use spin::Mutex;

lazy_static::lazy_static! {
    pub static ref SERIAL1: Mutex<SerialPort> = Mutex::new(unsafe {
        let mut serial_port = SerialPort::new(0x3F8);
        serial_port.init();
        serial_port
    });
}


#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => {
        $crate::serial_print!(concat!($fmt, "\n"), $($arg)*)
    };
}
