use super::SolvedLayout;

#[derive(Clone, Copy, Debug, Default)]
pub struct Anchor {
	pub x: f32,
	pub y: f32,
}

impl Anchor {
	pub const fn new(x: f32, y: f32) -> Self {
		Self { x, y }
	}

	pub const TOP_LEFT: Anchor = Anchor::new(0.0, 0.0);
	pub const TOP_CENTER: Anchor = Anchor::new(0.5, 0.0);
	pub const TOP_RIGHT: Anchor = Anchor::new(1.0, 0.0);
	pub const CENTER_LEFT: Anchor = Anchor::new(0.0, 0.5);
	pub const CENTER: Anchor = Anchor::new(0.5, 0.5);
	pub const CENTER_RIGHT: Anchor = Anchor::new(1.0, 0.5);
	pub const BOTTOM_LEFT: Anchor = Anchor::new(0.0, 1.0);
	pub const BOTTOM_CENTER: Anchor = Anchor::new(0.5, 1.0);
	pub const BOTTOM_RIGHT: Anchor = Anchor::new(1.0, 1.0);

	pub fn calc_child_abs_pos(
		&self,
		width: f32,
		height: f32,
		parent_layout: &SolvedLayout,
	) -> SolvedLayout {
		let x_offset = (parent_layout.width() - width) * self.x;
		let y_offset = (parent_layout.height() - height) * self.y;

		SolvedLayout {
			x: parent_layout.x + x_offset,
			y: parent_layout.y + y_offset,
			width,
			height,
		}
	}
}
