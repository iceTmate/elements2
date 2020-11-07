use crate::prelude::*;

pub enum ClientMode {
	Lobby,
	InGame {
		player_id: usize,
		world: World
	},
}

pub struct Client<B: Backend> {
	gamepad_state: RawGamepadState,
	socket: B::SocketBackend,
	mode: ClientMode,
}

impl<B: Backend> Client<B> {
	pub fn new(server_ip: &str) -> Client<B> {
		Client {
			gamepad_state: RawGamepadState::new(),
			socket: B::SocketBackend::new(server_ip),
			mode: ClientMode::Lobby,
		}
	}

	pub fn tick(&mut self, app: &mut App<B>) {
		match &mut self.mode {
			ClientMode::Lobby => {
				if let Some(Go { your_player_id, tilemap_image }) = self.socket.try_recv() {
					self.mode = ClientMode::InGame {
						player_id: your_player_id,
						world: World::new(0, &tilemap_image),
					};
				}
			},
			ClientMode::InGame { player_id, world } => {
				// receive packets
				if let Some(update) = self.socket.try_recv::<WorldUpdate>() {
					world.apply_update_within_app(update, app);
				}

				// handle inputs
				world.players[*player_id].input.update_gamepad(&self.gamepad_state);
				world.players[*player_id].input.update_peripherals(&app.peripherals_state);

				// send packets
				self.socket.send(&world.players[*player_id].input);

				// tick world
				world.tick_within_app(app);
			}
		}
	}

	pub fn draw(&mut self, app: &mut App<B>) {
		match &self.mode {
			ClientMode::Lobby => (), // TODO: drawing in lobby phase
			ClientMode::InGame { world, .. } => {
				let mut draw = Draw::new();
				world.draw(&mut draw);
				app.graphics_backend.draw(draw, Some(&world));
			}
		}
	}
}
