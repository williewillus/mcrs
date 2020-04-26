#[macro_use]
mod macros;

pub mod connection;
mod handshake;
mod login;
mod packet;
pub mod play;
pub mod proto;
mod status;

use crate::server::Server;
use std::net::TcpListener;
use std::sync::Arc;
use std::thread;

fn accept_loop(server: Arc<Server>)  {
    let listener = TcpListener::bind("127.0.0.1:25565").expect("Failed to bind socket");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                log::info!("Accepting connection to {:?}", stream.peer_addr());
                let mut conn = connection::Connection::new(stream, Arc::clone(&server));
                thread::spawn(move || {
                    match conn.process() {
                        Ok(()) => log::info!("stream disconnected successfully"),
                        Err(e) => log::error!("error processing connection: {}", e)
                    }
                });
            },
            Err(e) => {
                log::error!("Failed to accept new connection: {}", e);
            }
        }
    }
}

/// Spawns the accept loop thread and returns a handle to it
pub fn spawn_accept_loop(server: Arc<Server>) -> std::io::Result<thread::JoinHandle<()>> {
    thread::Builder::new()
        .name("accept loop".to_owned())
        .spawn(move || accept_loop(server))
}
