use crate::ecs::*;
use miniquad::*;

#[derive(Default)]
pub struct App {
	entities: Vec<Box<dyn ComponentVec>>,
}
impl App {
	pub fn add_entity(&mut self) {}
	pub fn run(&self) {}
}

impl EventHandler for App {
	fn update(&mut self, _skia_ctx: &mut skia::SkiaContext) {}

	fn mouse_button_down_event(
		&mut self,
		_skia_ctx: &mut skia::SkiaContext,
		_button: MouseButton,
		_x: f32,
		_y: f32,
	) {
	}

	fn mouse_button_up_event(
		&mut self,
		_skia_ctx: &mut skia::SkiaContext,
		_button: MouseButton,
		_x: f32,
		_y: f32,
	) {
	}

	fn mouse_motion_event(&mut self, _skia_ctx: &mut skia::SkiaContext, _x: f32, _y: f32) {}

	fn touch_event(
		&mut self,
		skia_ctx: &mut skia::SkiaContext,
		phase: TouchPhase,
		_id: u64,
		x: f32,
		y: f32,
	) {
	}

	fn resize_event(&mut self, skia_ctx: &mut skia::SkiaContext, width: f32, height: f32) {
		skia_ctx.recreate_surface(width as i32, height as i32);
	}

	fn draw(&mut self, skia_ctx: &mut skia::SkiaContext) {}
}
