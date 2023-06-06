#[derive(Clone, Copy, Debug)]
pub struct MousePosition {
	pub x: f32,
	pub y: f32,
}

#[derive(Clone, Copy, Debug)]
pub enum Event {
	MouseDown(MousePosition),
	MouseUp(MousePosition),
	MouseMove(MousePosition),
	MouseIn,
	MouseOut,
}
