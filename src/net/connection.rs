use std::net::TcpStream;

pub enum State {
    Handshake,
    Status,
    Login,
    Play,
}

pub struct Connection {
    state: State,
    stream: TcpStream,
}

impl Connection {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            state: State::Handshake,
            stream,
        }
    }

    pub fn process(&self) {}
}
