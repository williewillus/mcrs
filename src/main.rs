#![allow(dead_code)]

mod net;
mod server;
mod types;

use std::sync::Arc;

fn main() -> anyhow::Result<()> {
    simple_logger::init_by_env();
    log::info!("mcrs initializing");

    let server = Arc::new(server::Server::new());
    let accept_handle = net::spawn_accept_loop(Arc::clone(&server))?;
    
    accept_handle.join().unwrap();
    Ok(())
}
