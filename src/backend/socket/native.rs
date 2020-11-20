use crate::prelude::*;

pub struct NativeSocketBackend(UdpSocket);

impl SocketBackend for NativeSocketBackend {
	fn new(server_ip: &str) -> Self {
		let socket = UdpSocket::bind("0.0.0.0:0").expect("Could not create client socket");
		socket.set_nonblocking(true).unwrap();
		socket.connect((server_ip, PORT)).expect("Could not connect to server");

		let mut socket = NativeSocketBackend(socket);

		// this only happens on native!
		socket.send(&Init::Init);

		socket
	}

	fn is_open(&mut self) -> bool { true }

	fn send(&mut self, packet: &impl Packet) {
		send_packet(&mut self.0, packet);
	}

	fn try_recv<P: Packet>(&mut self) -> Option<P> {
		recv_packet(&mut self.0)
			.map(|(x, _)| x)
	}
}
