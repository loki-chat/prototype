use std::io;

use miniquad::skia::SkiaContext;
use skia_safe::{Color, Paint, Rect};

use crate::indentation;
use crate::layout::{DimScalar, Direction, FlexLayout, Layout, Padding, SolvedLayout};
use crate::widget::{Event, Widget};

pub struct PaneChild {
	solved_layout: SolvedLayout,
	widget: Box<dyn Widget>,
}

#[derive(Default)]
pub struct Pane {
	layout: Layout,
	padding: Padding,
	flex_layout: Option<FlexLayout>,
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
		self.children.push(PaneChild {
			solved_layout: SolvedLayout::default(),
			widget,
		});
	}

	pub fn pop_child(&mut self) -> Option<Box<dyn Widget>> {
		self.children.pop().map(|child| child.widget)
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
			// With flex-layout, all children are placed next to each other, vertically or horizontally.

			match flex_layout.direction {
				Direction::Horizontal => {
					let fills_count = (self.children.iter())
						.filter(|child| child.widget.layout().width.is_fill())
						.count();

					let filling_width = if fills_count == 0 {
						0.
					} else {
						let fixed_width: f32 = (self.children.iter())
							.filter_map(|child| match child.widget.layout().width {
								DimScalar::Fill => None,
								DimScalar::Hug => Some(child.widget.min_width()),
								DimScalar::Fixed(w) => Some(w),
							})
							.sum();

						let leftover_width = (inner_layout.width() - fixed_width).max(0.);
						leftover_width / fills_count as f32
					};

					let mut x = inner_layout.x_start();
					for child in &mut self.children {
						let child_width = match child.widget.layout().width {
							DimScalar::Fill => filling_width,
							DimScalar::Hug => child.widget.min_width(),
							DimScalar::Fixed(w) => w,
						};

						let inner_layout = inner_layout.with_width(child_width).with_x(x);
						child.solved_layout = child.widget.solve_layout(&inner_layout);

						x += child_width;
					}
				}
				Direction::Vertical => {
					let fills_count = (self.children.iter())
						.filter(|child| child.widget.layout().height.is_fill())
						.count();

					let filling_height = if fills_count == 0 {
						0.
					} else {
						let fixed_height: f32 = (self.children.iter())
							.filter_map(|child| match child.widget.layout().height {
								DimScalar::Fill => None,
								DimScalar::Hug => Some(child.widget.min_height()),
								DimScalar::Fixed(w) => Some(w),
							})
							.sum();

						let leftover_height = (inner_layout.height() - fixed_height).max(0.);
						leftover_height / fills_count as f32
					};

					let mut y = inner_layout.y_start();
					for child in &mut self.children {
						let child_height = match child.widget.layout().height {
							DimScalar::Fill => filling_height,
							DimScalar::Hug => child.widget.min_height(),
							DimScalar::Fixed(w) => w,
						};

						let inner_layout = inner_layout.with_height(child_height).with_y(y);
						child.solved_layout = child.widget.solve_layout(&inner_layout);

						y += child_height;
					}
				}
			}
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

		if let Some(flex_layout) = self.flex_layout.as_ref() {
			if flex_layout.direction == Direction::Horizontal {
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

		if let Some(flex_layout) = self.flex_layout.as_ref() {
			if flex_layout.direction == Direction::Vertical {
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

	fn draw(&self, skia_ctx: &mut SkiaContext, layout: &SolvedLayout) {
		let canvas = skia_ctx.surface.canvas();

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

		for child in &self.children {
			child.widget.draw(skia_ctx, &child.solved_layout);
		}
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		let mut handled = false;

		let should_handle = match event {
			Event::Clicked(x, y) => layout.contains(x, y),
			_ => true,
		};

		if should_handle {
			for child in &mut self.children {
				handled |= child.widget.handle_event(event, &child.solved_layout);

				if handled {
					break;
				}
			}
		}

		handled
	}
}
