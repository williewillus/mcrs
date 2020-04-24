#![allow(dead_code)]

mod net;
mod types;

fn main() -> anyhow::Result<()> {
    simple_logger::init_by_env();
    log::info!("mcrs initializing");

    let accept_handle = net::spawn_accept_loop()?;
    
    accept_handle.join().unwrap();
    Ok(())
}
