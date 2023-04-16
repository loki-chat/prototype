use miniquad::skia::SkiaContext;

use crate::layout::{DimScalar, Layout, SolvedLayout};

#[derive(Clone, Copy, Debug)]
pub enum Event {
	Clicked(f32, f32),
	HoverStart,
	HoverEnd,
}

pub trait Widget {
	fn layout(&self) -> &Layout;

	fn default_solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		let width = match self.layout().width {
			DimScalar::Fill => parent_layout.width(),
			DimScalar::Hug => self.min_width(),
			DimScalar::Fixed(w) => w,
		};

		let height = match self.layout().height {
			DimScalar::Fill => parent_layout.height(),
			DimScalar::Hug => self.min_height(),
			DimScalar::Fixed(h) => h,
		};

		(self.layout().anchor).calc_child_abs_pos(width, height, parent_layout)
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout;

	/// Minimum possible width in case we choose DimScalar::Hug as the layout width.
	fn min_width(&mut self) -> f32 {
		0.
	}

	/// Minimum possible height in case we choose DimScalar::Hug as the layout height.
	fn min_height(&mut self) -> f32 {
		0.
	}

	fn draw(&self, skia_ctx: &mut SkiaContext, layout: &SolvedLayout);
	fn handle_event(&mut self, event: Event) -> bool;
}
