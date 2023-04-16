use super::SolvedLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum HAnchor {
	Left,
	#[default]
	Center,
	Right,
}

impl HAnchor {
	pub fn calc_x_offset(&self, inner_width: f32, outer_width: f32) -> f32 {
		match self {
			HAnchor::Left => 0.,
			HAnchor::Center => (outer_width - inner_width) / 2.,
			HAnchor::Right => outer_width - inner_width,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum VAnchor {
	Top,
	#[default]
	Center,
	Bottom,
}

impl VAnchor {
	pub fn calc_y_offset(&self, inner_height: f32, outer_height: f32) -> f32 {
		match self {
			VAnchor::Top => 0.,
			VAnchor::Center => (outer_height - inner_height) / 2.,
			VAnchor::Bottom => outer_height - inner_height,
		}
	}
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct Anchor {
	pub v: VAnchor,
	pub h: HAnchor,
}

impl Anchor {
	pub const TOP_LEFT: Anchor = Anchor {
		v: VAnchor::Top,
		h: HAnchor::Left,
	};
	pub const TOP_CENTER: Anchor = Anchor {
		v: VAnchor::Top,
		h: HAnchor::Center,
	};
	pub const TOP_RIGHT: Anchor = Anchor {
		v: VAnchor::Top,
		h: HAnchor::Right,
	};

	pub const CENTER_LEFT: Anchor = Anchor {
		v: VAnchor::Center,
		h: HAnchor::Left,
	};
	pub const CENTER: Anchor = Anchor {
		v: VAnchor::Center,
		h: HAnchor::Center,
	};
	pub const CENTER_RIGHT: Anchor = Anchor {
		v: VAnchor::Center,
		h: HAnchor::Right,
	};

	pub const BOTTOM_LEFT: Anchor = Anchor {
		v: VAnchor::Bottom,
		h: HAnchor::Left,
	};
	pub const BOTTOM_CENTER: Anchor = Anchor {
		v: VAnchor::Bottom,
		h: HAnchor::Center,
	};
	pub const BOTTOM_RIGHT: Anchor = Anchor {
		v: VAnchor::Bottom,
		h: HAnchor::Right,
	};

	pub fn calc_child_abs_pos(
		&self,
		width: f32,
		height: f32,
		parent_layout: &SolvedLayout,
	) -> SolvedLayout {
		let x = self.h.calc_x_offset(width, parent_layout.width());
		let y = self.v.calc_y_offset(height, parent_layout.height());

		SolvedLayout {
			x: parent_layout.x + x,
			y: parent_layout.y + y,
			width,
			height,
		}
	}
}
