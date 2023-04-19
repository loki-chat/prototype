use std::io;

use skia_safe::Canvas;

use crate::layout::{DimScalar, Layout, SolvedLayout};

#[derive(Clone, Copy, Debug)]
pub enum Event {
	Clicked(f32, f32),
	HoverStart,
	HoverEnd,
}

pub trait Widget {
	fn layout(&self) -> &Layout;

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout;

	/// Minimum possible width in case we choose DimScalar::Hug as the layout width.
	fn min_width(&self) -> f32;

	/// Minimum possible height in case we choose DimScalar::Hug as the layout height.
	fn min_height(&self) -> f32;

	fn debug(&self, _w: &mut dyn io::Write, _deepness: usize) -> io::Result<()> {
		Ok(())
	}

	fn draw(&self, canvas: &mut Canvas, layout: &SolvedLayout);
	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool;
}

/// Partially resolve this widget's width.
///
/// - If the width is `Fill`, it returns `None`.
/// - If the width is `Hug`, it returns this widget's minimum width.
/// - If the width is `Fixed`, it returns that fixed width value.
pub fn solve_width(widget: &dyn Widget) -> Option<f32> {
	match widget.layout().width {
		DimScalar::Fill => None,
		DimScalar::Hug => Some(widget.min_width()),
		DimScalar::Fixed(w) => Some(w),
	}
}

/// Partially resolve this widget's height.
///
/// - If the height is `Fill`, it returns `None`.
/// - If the height is `Hug`, it returns this widget's minimum height.
/// - If the height is `Fixed`, it returns that fixed height value.
pub fn solve_height(widget: &dyn Widget) -> Option<f32> {
	match widget.layout().height {
		DimScalar::Fill => None,
		DimScalar::Hug => Some(widget.min_height()),
		DimScalar::Fixed(w) => Some(w),
	}
}

/// Default function to solve a widget's layout, based on the parent's solved layout.
pub fn default_solve_layout(widget: &mut impl Widget, parent_layout: &SolvedLayout) -> SolvedLayout {
	let width = solve_width(widget).unwrap_or_else(|| parent_layout.width());
	let height = solve_height(widget).unwrap_or_else(|| parent_layout.height());

	let (x, y) = parent_layout.point_at_anchor(widget.layout().anchor);
	SolvedLayout::from_origin(widget.layout().origin, x, y, width, height)
}
