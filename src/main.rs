use serialport::prelude::*;
use std::io::{self, Write};
use std::time::Duration;

fn main() {
    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(10);

    let port_name = "/dev/ttyACM0";
    let baud_rate = 9600;
    settings.baud_rate = baud_rate;
    match serialport::open_with_settings(&port_name, &settings) {
        Ok(mut port) => {
            let mut serial_buf: Vec<u8> = vec![0; 1000];
            println!("Receiving data on {} at {} baud:", &port_name, &baud_rate);
            loop {
                match port.read(serial_buf.as_mut_slice()) {
                    Ok(t) => {
                        io::stdout().write_all(&serial_buf[..t]).unwrap();
                        match std::str::from_utf8(&serial_buf) {
                            Ok(t) => {
                                if t.starts_with("1") {
                                    hit_snack();
                                }
                            }
                            Err(e) => eprintln!("error parsing UTF string: {:?}", e),
                        }
                    }
                    Err(ref e) if e.kind() == io::ErrorKind::TimedOut => (),
                    Err(e) => eprintln!("{:?}", e),
                }
            }
        }
        Err(e) => {
            eprintln!("Failed to open \"{}\". Error: {}", port_name, e);
            ::std::process::exit(1);
        }
    }
}

fn hit_snack() {
    println!("Hitting snack!");
}
