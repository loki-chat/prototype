use std::io;

use skia_safe::Canvas;

use crate::events::{Event, MousePosition};
use crate::indentation;
use crate::layout::{DimScalar, Direction, Flex, Layout, SolvedLayout, Padding};
use crate::widget::{default_solve_layout, solve_height, solve_width, Widget, WidgetContainer};

struct PaneChild {
	widget: Box<dyn Widget>,
	solved_layout: SolvedLayout,
	is_hovered: bool,
}

impl PaneChild {
	fn new(widget: Box<dyn Widget>) -> Self {
		Self {
			widget,
			solved_layout: SolvedLayout::default(),
			is_hovered: false,
		}
	}
}

#[derive(Default)]
pub struct Pane {
	layout: Layout,
	padding: Padding,
	flex: Option<Flex>,
	children: Vec<PaneChild>,
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

	pub fn with_flex(mut self, flex: Flex) -> Self {
		self.flex = Some(flex);
		self
	}
}

impl WidgetContainer for Pane {
	fn add_dyn_child(&mut self, widget: Box<dyn Widget>) {
		self.children.push(PaneChild::new(widget));
	}

	fn pop_child(&mut self) -> Option<Box<dyn Widget>> {
		self.children.pop().map(|child| child.widget)
	}
}

impl Widget for Pane {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		let layout = default_solve_layout(self, parent_layout);

		let inner_layout = layout.padded(self.padding);
		if let Some(flex) = &self.flex {
			solve_flex_layout(flex, &mut self.children, inner_layout);
		} else {
			// Without flex-layout, all children are superposed to each other.

			for child in &mut self.children {
				child.solved_layout = child.widget.solve_layout(&inner_layout);
			}
		}

		layout
	}

	fn min_width(&self) -> f32 {
		let width_pad = self.padding.left + self.padding.right;

		if let Some(flex) = self.flex.as_ref() {
			if flex.direction == Direction::Horizontal {
				let inner_min_width: f32 = (self.children.iter())
					.map(|child| match child.widget.layout().width {
						DimScalar::Fixed(w) => w,
						_ => child.widget.min_width(),
					})
					.sum();

				return inner_min_width + width_pad;
			}
		}

		let inner_min_width = (self.children.iter())
			.map(|child| child.widget.min_width())
			.max_by(|x, y| x.total_cmp(y))
			.unwrap_or_default();

		inner_min_width + width_pad
	}

	fn min_height(&self) -> f32 {
		let height_pad = self.padding.top + self.padding.bottom;

		if let Some(flex) = self.flex.as_ref() {
			if flex.direction == Direction::Vertical {
				let inner_min_height: f32 = (self.children.iter())
					.map(|child| match child.widget.layout().height {
						DimScalar::Fixed(h) => h,
						_ => child.widget.min_height(),
					})
					.sum();

				return inner_min_height + height_pad;
			}
		}

		let inner_min_height = (self.children.iter())
			.map(|child| child.widget.min_height())
			.max_by(|x, y| x.total_cmp(y))
			.unwrap_or_default();

		inner_min_height + height_pad
	}

	fn debug(&self, w: &mut dyn io::Write, deepness: usize) -> io::Result<()> {
		writeln!(w, "{}<pane>", indentation(deepness))?;
		for child in &self.children {
			child.widget.debug(w, deepness + 1)?;
		}
		writeln!(w, "{}</pane>", indentation(deepness))
	}

	fn draw(&self, canvas: &mut Canvas, _layout: &SolvedLayout) {
		for child in &self.children {
			child.widget.draw(canvas, &child.solved_layout);
		}
	}

	fn handle_event(&mut self, event: Event, _layout: &SolvedLayout) -> bool {
		match event {
			Event::MouseDown(MousePosition { x, y }) | Event::MouseUp(MousePosition { x, y }) => {
				for child in &mut self.children {
					if !child.solved_layout.contains(x, y) {
						continue;
					}

					child.widget.handle_event(event, &child.solved_layout);
				}
			}
			Event::MouseMove(MousePosition { x, y }) => {
				for child in &mut self.children {
					if child.solved_layout.contains(x, y) {
						if !child.is_hovered {
							child.is_hovered = true;
							(child.widget).handle_event(Event::MouseIn, &child.solved_layout);
						}
						child.widget.handle_event(event, &child.solved_layout);
					} else if child.is_hovered {
						child.is_hovered = false;
						(child.widget).handle_event(Event::MouseOut, &child.solved_layout);
					}
				}
			}
			_ => (),
		}

		true
	}
}

/// Solves a pane's children's solved layouts with a flex layout.
///
/// With a flex layout, all children are placed next to each other, vertically or horizontally.
fn solve_flex_layout(
	flex: &Flex,
	children: &mut [PaneChild],
	inner_layout: SolvedLayout,
) {
	match flex.direction {
		Direction::Horizontal => {
			let fills_count = (children.iter())
				.filter(|child| child.widget.layout().width.is_fill())
				.count();

			let filling_width = if fills_count == 0 {
				0.
			} else {
				let fixed_width: f32 = (children.iter())
					.filter_map(|child| solve_width(child.widget.as_ref()))
					.sum();

				let gap_width = flex.gap * children.len().saturating_sub(1) as f32;
				let leftover_width = (inner_layout.width() - fixed_width - gap_width).max(0.);
				leftover_width / fills_count as f32
			};

			let mut x = inner_layout.x_start();
			for child in children.iter_mut() {
				// Each child is given a slice of the inner layout.

				let child_width = solve_width(child.widget.as_ref()).unwrap_or(filling_width);
				let inner_layout = inner_layout.with_width(child_width).with_x(x);
				child.solved_layout = child.widget.solve_layout(&inner_layout);

				x += child_width + flex.gap;
			}
		}
		Direction::Vertical => {
			// maybe put this into a function since it's
			// copy-pasted from the Horizontal case but for height?

			let fills_count = (children.iter())
				.filter(|child| child.widget.layout().height.is_fill())
				.count();

			let filling_height = if fills_count == 0 {
				0.
			} else {
				let fixed_height: f32 = (children.iter())
					.filter_map(|child| solve_height(child.widget.as_ref()))
					.sum();

				let gap_width = flex.gap * children.len().saturating_sub(1) as f32;
				let leftover_height = (inner_layout.height() - fixed_height - gap_width).max(0.);
				leftover_height / fills_count as f32
			};

			let mut y = inner_layout.y_start();
			for child in children.iter_mut() {
				let child_height = solve_height(child.widget.as_ref()).unwrap_or(filling_height);
				let inner_layout = inner_layout.with_height(child_height).with_y(y);
				child.solved_layout = child.widget.solve_layout(&inner_layout);

				y += child_height + flex.gap;
			}
		}
	}
}
