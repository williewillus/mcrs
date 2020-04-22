#![allow(dead_code)]

mod net;

use log::{info, error};
use net::connection::Connection;
use std::net::TcpListener;
use std::thread;

fn main() -> anyhow::Result<()> {
    simple_logger::init_by_env();
    info!("mcrs initializing");

    let listener = TcpListener::bind("127.0.0.1:25565").expect("Failed to bind socket");
    for stream in listener.incoming() {
        let stream = stream?;
        info!("Accepting connection to {:?}", stream.peer_addr());
        let mut conn = Connection::new(stream);
        thread::spawn(move || {
            match conn.process() {
                Ok(()) => info!("stream disconnected successfully"),
                Err(e) => error!("error processing connection: {}", e)
            }
        });
    }
    Ok(())
}
