#![allow(dead_code)]

mod net;

use net::connection::Connection;
use std::net::TcpListener;
use std::thread;

fn main() -> anyhow::Result<()> {
    println!("Hello, world!");

    let listener = TcpListener::bind("127.0.0.1:25565").expect("Failed to bind socket");
    for stream in listener.incoming() {
        let stream = stream?;
        println!("Accepting connection to {:?}", stream.peer_addr());
        let mut conn = Connection::new(stream);
        thread::spawn(move || {
            match conn.process() {
                Ok(()) => println!("stream disconnected successfully"),
                Err(e) => eprintln!("error processing connection: {}", e)
            }
        });
    }
    Ok(())
}
