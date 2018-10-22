extern crate bus;
extern crate clap;
extern crate serialport;

use bus::Bus;
use bus::BusReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::io::{self, Write};

use serialport::prelude::*;


use clap::{App, Arg};

use std::sync::*;

type StrRead = BusReader<Vec<u8>>;

fn handle_connection(mut stream: TcpStream, mut ser_rx: StrRead )
{

    loop{
        let data = ser_rx.recv().unwrap();

        if stream.write( &data ).is_ok() {
            stream.flush().unwrap();
        } else {
            break;
        }
    }

}

fn setup() ->  clap::ArgMatches<'static> {
     let matches = App::new("snet")
        .version("1.0")
        .author("Kristoffer Ödmark")
        .about("Forward serialport bytes to tcp connections")
            .arg(Arg::with_name("serialport")
                .help("The serialport to read from")
                .takes_value(true)
                .short("s")
                .long("serial")
                .default_value("/dev/ttyUSB0")
                )
            .arg(Arg::with_name("baudrate")
                .help("baud rate of the serialport")
                .takes_value(true)
                .short("b")
                .long("baud")
                .required(false)
                .default_value("115200")
                )
            .arg(Arg::with_name("port")
                .help("The server port to connect to")
                .takes_value(true)
                .short("p")
                .long("port")
                .required(true)
                .default_value("2816")
                )
        .get_matches();
     matches
}

fn main() {

    let matches = setup();


    let baud_rate = matches.value_of("baudrate").unwrap().parse().unwrap();
    let portname = matches.value_of("serialport").unwrap();

    let port = matches.value_of("port").unwrap();

    let mut settings: SerialPortSettings = Default::default();
    settings.timeout = Duration::from_millis(5);
    settings.baud_rate =baud_rate;

    // Multiple consumer, single producer
    let bus: Bus<Vec<u8>> = Bus::new(10);
    // We are going to let different threads spawn rx and send broadcasts
    let bus = Arc::new( Mutex::new( bus ) );

    let listener = TcpListener::bind(["127.0.0.1:", port].concat() ).expect("Could not bind to port");

    let serialport = serialport::open_with_settings(&portname, &settings);

    let mut serialport = serialport.expect("Could not connect to serialport");

    let bus_add = bus.clone();
    thread::spawn(move  || {
        for stream in listener.incoming() {
            let stream = stream.unwrap();
            let rx = bus_add.lock().unwrap().add_rx();
            thread::spawn( || {
                handle_connection(stream, rx );
            });

        }
    });

    let handle = thread::spawn(move || {
        loop{
            let mut serial_bytes  = [0;10000];
            match serialport.read( &mut serial_bytes[..] ) {
                Ok(n) => {
                    bus.lock().unwrap().broadcast(serial_bytes[..n].to_vec());
                },
                // Timeouts just means there was no bytes written during this time
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {
                },
                // ends up here if unplugged
                Err(e) => {
                    println!("{}",e);
                    break;
                },
            }
        }
    });
    // Exit if the serialport errors out on us
    let _exit = handle.join();
}
