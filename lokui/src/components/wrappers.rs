use std::ops::{Deref, DerefMut};

use skia_safe::{Canvas, Paint, RRect, Rect};

use crate::anim::Property;
use crate::events::Event;
use crate::layout::{Layout, Padding, SolvedLayout};
use crate::state::{Color, Lazy};
use crate::widget::Widget;

pub struct WithPadding<W: Widget> {
	widget: W,
	padding: Padding,
}

impl<W: Widget> Widget for WithPadding<W> {
	fn layout(&self) -> &Layout {
		self.widget.layout()
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		self.widget.solve_layout(parent_layout)
	}

	fn min_width(&self) -> f32 {
		self.widget.min_width() + self.padding.left + self.padding.right
	}

	fn min_height(&self) -> f32 {
		self.widget.min_height() + self.padding.top + self.padding.bottom
	}

	fn draw(&self, canvas: &mut Canvas, layout: &SolvedLayout) {
		self.widget.draw(canvas, layout);
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		self.widget.handle_event(event, layout)
	}
}

pub struct BackgroundState {
	pub color: Property<Color>,
	pub border_radius: Property<f32>,
	pub stroke: Option<(Property<Color>, Property<f32>)>,
}

impl BackgroundState {
	pub fn new(color: Color, border_radius: f32, stroke: Option<(Color, f32)>) -> Self {
		Self {
			color: Property::new(color),
			border_radius: Property::new(border_radius),
			stroke: stroke.map(|(c, w)| (Property::new(c), Property::new(w))),
		}
	}
}

pub struct WithBackground<W: Widget> {
	pub(crate) widget: W,
	pub(crate) state: Lazy<BackgroundState>,
}

impl<W: Widget> Deref for WithBackground<W> {
	type Target = W;

	fn deref(&self) -> &Self::Target {
		&self.widget
	}
}

impl<W: Widget> DerefMut for WithBackground<W> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.widget
	}
}

impl<W: Widget> Widget for WithBackground<W> {
	fn layout(&self) -> &Layout {
		self.widget.layout()
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		self.widget.solve_layout(parent_layout)
	}

	fn min_width(&self) -> f32 {
		self.widget.min_width()
	}

	fn min_height(&self) -> f32 {
		self.widget.min_height()
	}

	fn draw(&self, canvas: &mut Canvas, layout: &SolvedLayout) {
		let rect = Rect::from_xywh(
			layout.x_start(),
			layout.y_start(),
			layout.width(),
			layout.height(),
		);

		let mut state = self.state.get_mut();

		let radius = state.border_radius.current();
		let rect = RRect::new_rect_xy(rect, radius, radius);

		let mut paint = Paint::default();
		paint.set_anti_alias(true);

		paint.set_stroke(false);
		paint.set_color(state.color.current().into_skia());
		canvas.draw_rrect(rect, &paint);

		if let Some((color, width)) = &mut state.stroke {
			paint.set_stroke(true);
			paint.set_stroke_width(width.current());
			paint.set_color(color.current().into_skia());
			canvas.draw_rrect(rect, &paint);
		}

		self.widget.draw(canvas, layout);
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		self.widget.handle_event(event, layout)
	}
}
