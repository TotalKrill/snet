extern crate bus;

use bus::Bus;
use bus::BusReader;
use std::net::TcpListener;
use std::net::TcpStream;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;

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

fn main() {
    let bus: Bus<Vec<u8>> = Bus::new(10);

    let bus = Arc::new( Mutex::new( bus ) );

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

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

    // will be changed to a thread that uses the serialport
    let bus_broadcast = bus.clone();
    thread::spawn(move || {
        loop{
            bus_broadcast.lock().unwrap().broadcast(b"hej baberiba\n".to_vec());
            thread::sleep(Duration::from_millis(500));
        }
    });

    loop{
    };
}
