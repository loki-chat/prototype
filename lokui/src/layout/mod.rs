pub use anchor::*;
pub use padding::*;

pub mod anchor;
pub mod padding;

/// Scalar value used to represent static and dynamic widths and heights.
#[derive(Clone, Copy, Debug, Default)]
pub enum DimScalar {
	/// The widget fills its parent on that dimension.
	#[default]
	Fill,
	/// The widget hugs its internal content on that dimension.
	Hug,
	/// The dimension is fixed by that amount of pixels.
	Fixed(f32),
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
	/// Position offset in pixels on the `x` axis.
	pub x: f32,
	/// Position offset in pixels on the `y` axis.
	pub y: f32,
	/// Width of the widget box.
	pub width: DimScalar,
	/// Height of the widget box.
	pub height: DimScalar,
	/// Point where the widget is placed relative to its parent.
	pub anchor: Anchor,
	/// Point of origin of the widget relative to its own box.
	pub origin: Anchor,
}

impl Layout {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_pos(mut self, x: f32, y: f32) -> Self {
		self.x = x;
		self.y = y;
		self
	}

	pub fn with_dimension(mut self, width: DimScalar, height: DimScalar) -> Self {
		self.width = width;
		self.height = height;
		self
	}

	pub fn with_anchor(mut self, anchor: Anchor) -> Self {
		self.anchor = anchor;
		self
	}

	pub fn with_origin(mut self, origin: Anchor) -> Self {
		self.origin = origin;
		self
	}

	pub fn position(&self) -> (f32, f32) {
		(self.x, self.y)
	}

	pub fn dimension(&self) -> (DimScalar, DimScalar) {
		(self.width, self.height)
	}
}

/// State returned by a widget that solved its layout geometry.
#[derive(Clone, Copy, Debug, Default)]
pub struct SolvedLayout {
	/// Absolute X coordinate of the concrete widget box from the top-level corner.
	x: f32,
	/// Absolute Y coordinate of the concrete widget box from the top-level corner.
	y: f32,
	/// Width of the concrete widget box.
	width: f32,
	/// Height of the concrete widget box.
	height: f32,
}

impl SolvedLayout {
	pub fn from_top_left(x: f32, y: f32, width: f32, height: f32) -> Self {
		Self {
			x,
			y,
			width,
			height,
		}
	}

	pub fn from_2_points(xa: f32, ya: f32, xb: f32, yb: f32) -> Self {
		Self {
			x: xa.min(xb),
			y: ya.min(yb),
			width: (xb - xa).abs(),
			height: (yb - ya).abs(),
		}
	}

	pub fn x_start(&self) -> f32 {
		self.x
	}

	pub fn y_start(&self) -> f32 {
		self.y
	}

	pub fn x_end(&self) -> f32 {
		self.x + self.width
	}

	pub fn y_end(&self) -> f32 {
		self.y + self.height
	}

	pub fn width(&self) -> f32 {
		self.width
	}

	pub fn height(&self) -> f32 {
		self.height
	}

	pub fn contains_x(&self, x: f32) -> bool {
		x >= self.x && x <= self.x + self.width
	}

	pub fn contains_y(&self, y: f32) -> bool {
		y >= self.y && y <= self.y + self.height
	}

	pub fn contains(&self, x: f32, y: f32) -> bool {
		self.contains_x(x) && self.contains_y(y)
	}

	pub fn padded(&self, padding: Padding) -> Self {
		Self {
			x: self.x + padding.left,
			y: self.y + padding.top,
			width: self.width - padding.left - padding.right,
			height: self.height - padding.top - padding.bottom,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum Direction {
	#[default]
	Horizontal,
	Vertical,
}

#[derive(Clone, Debug, Default)]
pub struct FlexLayout {
	pub direction: Direction,
	pub align: Anchor,
	pub gap: f32,
}
