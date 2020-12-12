use crate::prelude::*;

pub struct ServerConnector<B: Backend> {
    pub socket: B::SocketBackend,
    pub request_send: bool,
    pub request_failed: bool,
}

impl<B: Backend> ServerConnector<B> {
    pub fn new(master_server_ip: &str) -> ServerConnector<B> {
        let mut socket = B::SocketBackend::new(master_server_ip, MASTER_SERVER_PORT);

        // socket.send(&Init::Init).unwrap();

        ServerConnector {
            socket,
            request_send: false,
            request_failed: false,
        }
    }

    pub fn tick(&mut self, _app: &mut App<B>) {
        if !self.request_send {
            match self.socket.send(&MasterServerPacket::ClientRequest { name: String::from("test player") }) {
                Ok(()) => {},
                Err(_e) => self.request_failed = true,
            }
            self.request_send = true;
        }
        if let Some(MasterClientPacket::GameRedirection(game_ip)) = self.socket.try_recv() {
            unimplemented!("TODO: instantiate client");
        }
    }

    pub fn draw(&mut self, _app: &mut App<B>) {
        // TODO: draw some status information
    }
}

