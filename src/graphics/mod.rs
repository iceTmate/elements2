mod world;
pub use world::*;

mod surface_vec;
pub use surface_vec::*;

mod context;
pub use context::*;

mod draw;
pub use draw::*;

use crate::prelude::*;
use draw_triangles::*;
use draw_tilemap::*;
use draw_fluidmap::*;

pub const SURFACE_FORMAT: wgpu::TextureFormat = wgpu::TextureFormat::Bgra8UnormSrgb;

pub struct Graphics {
	#[allow(unused)] instance: wgpu::Instance,
	surface: wgpu::Surface,
	device: wgpu::Device,
	queue: wgpu::Queue,
	window_size: Vec2u,
	swap_chain: wgpu::SwapChain,
	triangles: DrawTriangles,
	tilemap: DrawTilemap,
	fluidmap: DrawFluidmap,
}

fn create_swap_chain(device: &wgpu::Device, surface: &wgpu::Surface, size: Vec2u) -> wgpu::SwapChain {
	let swap_chain_desc = wgpu::SwapChainDescriptor {
		usage: wgpu::TextureUsage::OUTPUT_ATTACHMENT,
		format: SURFACE_FORMAT,
		width: size.x,
		height: size.y,
		present_mode: wgpu::PresentMode::Immediate,
	};

	device.create_swap_chain(surface, &swap_chain_desc)
}

impl Graphics {
	/* create instance
	 * create surface
	 * choose adapter (physical device)
	 * get device
	 */
	pub fn new(window: &win::Window) -> Graphics {
		let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);

		let surface = unsafe { instance.create_surface(window) };
		let adapter = futures::executor::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
			power_preference: wgpu::PowerPreference::HighPerformance,
			compatible_surface: Some(&surface)
		}))
			.unwrap();

		let (device, queue) = futures::executor::block_on(adapter.request_device(
			&wgpu::DeviceDescriptor {
				features: wgpu::Features::empty(),
				limits: Default::default(),
				shader_validation: true
			},
			None
		))
			.unwrap();

		let window_size = window.inner_size();
		let window_size = Vec2u::new(window_size.width, window_size.height);

		let swap_chain = create_swap_chain(&device, &surface, window_size);

		let triangles = DrawTriangles::new(&device, &queue);
		let tilemap = DrawTilemap::new(&device, &queue);
		let fluidmap = DrawFluidmap::new(&device, &queue);

		Graphics {
			instance,
			surface,
			device,
			queue,
			window_size,
			swap_chain,
			triangles,
			tilemap,
			fluidmap,
		}
	}

	pub fn create_draw(&self) -> Draw {
		Draw::new(
			self.window_size,
			self.triangles.texture_count(),
		)
	}

	pub fn draw(&mut self, draw: &mut Draw, world: &GraphicsWorld) {
		self.draw_players(draw, world);
		self.draw_cursors(draw, world);
		self.draw_healthbars(draw, world);
	}

	/* create and fill draw pass
	 * create and fill command buffer
	 * submit command buffer to queue
	 */
	pub fn flush(&mut self, draw: &Draw, world: &GraphicsWorld) {
		let swap_chain_texture = self.swap_chain
			.get_current_frame()
			.unwrap()
			.output;

		let mut encoder = self.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
			label: Some("command encoder")
		});

		let mut graphics_context = GraphicsContext {
			device: &self.device,
			queue: &self.queue,
			swap_chain_texture: &swap_chain_texture,
			encoder: &mut encoder,
		};

		let clear_color = wgpu::Color {
			r: 50.0 / 255.0,
			g: 120.0 / 255.0,
			b: 215.0 / 255.0,
			a: 1.0,
		};

		self.fluidmap.render(
			&mut graphics_context,
			wgpu::LoadOp::Clear(clear_color),
			&world,
		);

		self.tilemap.render(
			&mut graphics_context,
			wgpu::LoadOp::Load,
			world.tilemap_size,
			&world.tilemap_data,
		);

		self.triangles.render(
			&mut graphics_context,
			wgpu::LoadOp::Load,
			draw,
		);

		self.queue.submit(Some(encoder.finish()));
	}

	pub fn resize(&mut self, size: Vec2u) {
		self.window_size = size;
		self.swap_chain = create_swap_chain(&self.device, &self.surface, size);
	}
}
