use crate::prelude::*;

const REQUEST_SEND_INTERVAL: u32 = 120; // 2 seconds

pub struct ServerConnector<B: Backend> {
    pub socket: B::SocketBackend,
    pub request_send_counter: u32,
    pub request_failed: bool,
    pub game_ip: Option<(String, u16)>,
}

impl<B: Backend> ServerConnector<B> {
    pub fn new(master_server_ip: &str) -> ServerConnector<B> {
        let socket = B::SocketBackend::new(master_server_ip, MASTER_SERVER_PORT);

        ServerConnector {
            socket,
            request_send_counter: 0,
            request_failed: false,
            game_ip: None,
        }
    }

    pub fn tick(&mut self, _app: &mut App<B>) {
        if !self.socket.is_open() { return; }
        if !self.request_send_counter >= REQUEST_SEND_INTERVAL {
            match self.socket.send(&MasterServerPacket::ClientRequest { name: String::from("test player") }) {
                Ok(()) => {},
                Err(_e) => self.request_failed = true,
            }
            self.request_send = true;
        }
        self.request_send_counter += 1;
        if let Some(MasterClientPacket::GameRedirection(game_ip, port)) = self.socket.try_recv() {
            self.game_ip = Some((game_ip, port));
        }
    }

    pub fn draw(&mut self, _app: &mut App<B>) {
        // TODO: draw some status information
    }
}

