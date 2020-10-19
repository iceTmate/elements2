#![feature(drain_filter)]
#![feature(const_fn)]

include!("base.rs");

use crate::prelude::*;

#[cfg(feature = "native-client")]
fn main() {
	let server_arg = std::env::args().nth(1);
	if let Some("server") = server_arg.as_deref() {
		Server::new().run();
		return;
	}

	let (sender, receiver) = channel::<GraphicsWorld>();

	thread::spawn(move || {
		match server_arg.as_deref() {
			Some("menu") => App::new(sender).run_menu_and_game(),
			Some(ip) => App::new(sender).run_client(ip),
			None => App::new(sender).run_local(0),
		}
	});

	let event_loop = win::EventLoop::new();
	let window = win::WindowBuilder::new()
		.with_inner_size(win::PhysicalSize::new(1280, 720))
		.with_resizable(false)
		.with_title("Elements")
		.build(&event_loop)
		.unwrap();

	let mut graphics = Graphics::new(&window);

	event_loop.run(move |event, window_target, control_flow| {
		*control_flow = win::ControlFlow::Poll;

		match event {
			win::Event::WindowEvent {event: win::WindowEvent::CloseRequested, ..} => {
				*control_flow = win::ControlFlow::Exit;
			},
			win::Event::WindowEvent {event: win::WindowEvent::Resized(size), ..} => {
				graphics.resize(Vec2u::new(size.width, size.height));
			},
			win::Event::MainEventsCleared => {
				window.request_redraw();
			},
			win::Event::RedrawRequested {..} => {
				let graphics_world = receiver.recv().unwrap();
				graphics.draw(&graphics_world);
				graphics.flush(&graphics_world);
			},
			_ => ()
		}
	});
}

#[cfg(feature = "web-client")]
fn main() {
	panic!("web version does not have a main()!A")
}

#[cfg(not(feature = "client"))]
fn main() {
	Server::new().run();
}
