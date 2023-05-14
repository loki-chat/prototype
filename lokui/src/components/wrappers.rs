use std::ops::{Deref, DerefMut};

use skia_safe::{Canvas, Paint, RRect, Rect};

use crate::events::{Event, MousePosition};
use crate::layout::{Layout, SolvedLayout};
use crate::state::{Lazy, RectState};
use crate::widget::Widget;

// bg

pub struct WithBg<W> {
	pub(super) widget: W,
	pub(super) state: Lazy<RectState>,
}

impl<W> Deref for WithBg<W> {
	type Target = W;

	fn deref(&self) -> &Self::Target {
		&self.widget
	}
}

impl<W> DerefMut for WithBg<W> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.widget
	}
}

impl<W: Widget> Widget for WithBg<W> {
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

// onclick

pub struct WithOnClick<W> {
	pub(super) widget: W,
	pub(super) on_click: Box<dyn FnMut(f32, f32)>,
	pub(super) is_mouse_down: bool,
}

impl<W> Deref for WithOnClick<W> {
	type Target = W;

	fn deref(&self) -> &Self::Target {
		&self.widget
	}
}

impl<W> DerefMut for WithOnClick<W> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.widget
	}
}

impl<W: Widget> Widget for WithOnClick<W> {
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
		self.widget.draw(canvas, layout);
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		match event {
			Event::MouseDown(_) => {
				self.is_mouse_down = true;
			}
			Event::MouseUp(MousePosition { x, y }) => {
				if self.is_mouse_down {
					self.is_mouse_down = false;
					(self.on_click)(x, y);
				}
			}
			Event::MouseOut => self.is_mouse_down = false,
			_ => (),
		}

		self.widget.handle_event(event, layout)
	}
}

// onevent

pub struct WithOnEvent<W> {
	pub(super) widget: W,
	pub(super) callback: Box<dyn FnMut(Event) -> bool>,
}

impl<W> Deref for WithOnEvent<W> {
	type Target = W;

	fn deref(&self) -> &Self::Target {
		&self.widget
	}
}

impl<W> DerefMut for WithOnEvent<W> {
	fn deref_mut(&mut self) -> &mut Self::Target {
		&mut self.widget
	}
}

impl<W: Widget> Widget for WithOnEvent<W> {
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
		self.widget.draw(canvas, layout);
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		if !(self.callback)(event) {
			return self.widget.handle_event(event, layout);
		}

		false
	}
}
