use std::io;

use skia_safe::{Canvas, Color, Paint, Rect};

use crate::indentation;
use crate::layout::{DimScalar, Direction, FlexLayout, Layout, Padding, SolvedLayout};
use crate::widget::{Event, Widget};

#[derive(Default)]
pub struct Pane {
	layout: Layout,
	padding: Padding,
	flex_layout: Option<FlexLayout>,
	children: Vec<(Box<dyn Widget>, SolvedLayout)>,
}

pub fn pane() -> Pane {
	Pane::default()
}

impl Pane {
	pub fn with_layout(mut self, layout: Layout) -> Self {
		self.layout = layout;
		self
	}

	pub fn with_padding(mut self, padding: Padding) -> Self {
		self.padding = padding;
		self
	}

	pub fn with_flex_layout(mut self, flex_layout: FlexLayout) -> Self {
		self.flex_layout = Some(flex_layout);
		self
	}

	pub fn child(mut self, widget: impl Widget + 'static) -> Self {
		self.add_child(widget);
		self
	}

	pub fn add_child(&mut self, widget: impl Widget + 'static) {
		self.add_dyn_child(Box::new(widget));
	}

	pub fn add_dyn_child(&mut self, widget: Box<dyn Widget>) {
		self.children.push((widget, SolvedLayout::default()));
	}

	pub fn pop_child(&mut self) -> Option<Box<dyn Widget>> {
		self.children.pop().map(|(widget, _)| widget)
	}
}

impl Widget for Pane {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		let layout = self.default_solve_layout(parent_layout);

		let inner_layout = layout.padded(self.padding);
		if let Some(flex_layout) = &self.flex_layout {
			solve_flex_layout(flex_layout, &mut self.children, inner_layout);
		} else {
			// Without flex-layout, all children are superposed to each other.

			for (widget, solved_layout) in &mut self.children {
				*solved_layout = widget.solve_layout(&inner_layout);
			}
		}

		layout
	}

	fn min_width(&self) -> f32 {
		let width_pad = self.padding.left + self.padding.right;

		if let Some(flex_layout) = self.flex_layout.as_ref() {
			if flex_layout.direction == Direction::Horizontal {
				let inner_min_width: f32 = (self.children.iter())
					.map(|(widget, _)| match widget.layout().width {
						DimScalar::Fixed(w) => w,
						_ => widget.min_width(),
					})
					.sum();

				return inner_min_width + width_pad;
			}
		}

		let inner_min_width = (self.children.iter())
			.map(|(widget, _)| widget.min_width())
			.max_by(|x, y| x.total_cmp(y))
			.unwrap_or_default();

		inner_min_width + width_pad
	}

	fn min_height(&self) -> f32 {
		let height_pad = self.padding.top + self.padding.bottom;

		if let Some(flex_layout) = self.flex_layout.as_ref() {
			if flex_layout.direction == Direction::Vertical {
				let inner_min_height: f32 = (self.children.iter())
					.map(|(widget, _)| match widget.layout().height {
						DimScalar::Fixed(h) => h,
						_ => widget.min_height(),
					})
					.sum();

				return inner_min_height + height_pad;
			}
		}

		let inner_min_height = (self.children.iter())
			.map(|(widget, _)| widget.min_height())
			.max_by(|x, y| x.total_cmp(y))
			.unwrap_or_default();

		inner_min_height + height_pad
	}

	fn debug(&self, w: &mut dyn io::Write, deepness: usize) -> io::Result<()> {
		writeln!(w, "{}<pane>", indentation(deepness))?;
		for (widget, _) in &self.children {
			widget.debug(w, deepness + 1)?;
		}
		writeln!(w, "{}</pane>", indentation(deepness))
	}

	fn draw(&self, canvas: &mut Canvas, layout: &SolvedLayout) {
		let rect = Rect::from_xywh(
			layout.x_start(),
			layout.y_start(),
			layout.width(),
			layout.height(),
		);

		let mut paint = Paint::default();
		paint.set_anti_alias(true);
		paint.set_stroke_width(2.);

		paint.set_stroke(false);
		paint.set_color(Color::from(0x40_00cc51));
		canvas.draw_rect(rect, &paint);

		paint.set_stroke(true);
		paint.set_color(Color::from(0xff_00cc51));
		canvas.draw_rect(rect, &paint);

		for (widget, solved_layout) in &self.children {
			widget.draw(canvas, solved_layout);
		}
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		let mut handled = false;

		let should_handle = match event {
			Event::Clicked(x, y) => layout.contains(x, y),
			_ => true,
		};

		if should_handle {
			for (widget, solved_layout) in &mut self.children {
				handled |= widget.handle_event(event, solved_layout);

				if handled {
					break;
				}
			}
		}

		handled
	}
}

/// Solves a pane's children's solved layouts with a flex layout.
///
/// With a flex layout, all children are placed next to each other, vertically or horizontally.
fn solve_flex_layout(
	flex_layout: &FlexLayout,
	children: &mut [(Box<dyn Widget>, SolvedLayout)],
	inner_layout: SolvedLayout,
) {
	match flex_layout.direction {
		Direction::Horizontal => {
			let fills_count = (children.iter())
				.filter(|(widget, _)| widget.layout().width.is_fill())
				.count();

			let filling_width = if fills_count == 0 {
				0.
			} else {
				let fixed_width: f32 = (children.iter())
					.filter_map(|(widget, _)| match widget.layout().width {
						DimScalar::Fill => None,
						DimScalar::Hug => Some(widget.min_width()),
						DimScalar::Fixed(w) => Some(w),
					})
					.sum();

				let gap_width = flex_layout.gap * children.len().saturating_sub(1) as f32;
				let leftover_width = (inner_layout.width() - fixed_width - gap_width).max(0.);
				leftover_width / fills_count as f32
			};

			let mut x = inner_layout.x_start();
			for (widget, solved_layout) in children.iter_mut() {
				// Each child is given a slice of the inner layout.

				let child_width = match widget.layout().width {
					DimScalar::Fill => filling_width,
					DimScalar::Hug => widget.min_width(),
					DimScalar::Fixed(w) => w,
				};

				let inner_layout = inner_layout.with_width(child_width).with_x(x);
				*solved_layout = widget.solve_layout(&inner_layout);

				x += child_width + flex_layout.gap;
			}
		}
		Direction::Vertical => {
			// maybe put this into a function since it's
			// copy-pasted from the Horizontal case but for height?

			let fills_count = (children.iter())
				.filter(|(widget, _)| widget.layout().height.is_fill())
				.count();

			let filling_height = if fills_count == 0 {
				0.
			} else {
				let fixed_height: f32 = (children.iter())
					.filter_map(|(widget, _)| match widget.layout().height {
						DimScalar::Fill => None,
						DimScalar::Hug => Some(widget.min_height()),
						DimScalar::Fixed(w) => Some(w),
					})
					.sum();

				let gap_width = flex_layout.gap * children.len().saturating_sub(1) as f32;
				let leftover_height = (inner_layout.height() - fixed_height - gap_width).max(0.);
				leftover_height / fills_count as f32
			};

			let mut y = inner_layout.y_start();
			for (widget, solved_layout) in children.iter_mut() {
				let child_height = match widget.layout().height {
					DimScalar::Fill => filling_height,
					DimScalar::Hug => widget.min_height(),
					DimScalar::Fixed(w) => w,
				};

				let inner_layout = inner_layout.with_height(child_height).with_y(y);
				*solved_layout = widget.solve_layout(&inner_layout);

				y += child_height + flex_layout.gap;
			}
		}
	}
}
