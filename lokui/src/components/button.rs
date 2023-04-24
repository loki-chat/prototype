use std::fmt::Display;
use std::io;
use std::time::Duration;

use skia_safe::{Canvas, Paint, Rect};

use crate::anim::{ease, Property};
use crate::events::{Event, MousePosition};
use crate::indentation;
use crate::layout::{Layout, Padding, SolvedLayout};
use crate::state::{lazy, Color, Lazy};
use crate::widget::{default_solve_layout, Widget};

use super::text::Text;

pub struct Button<T: Display> {
	layout: Layout,
	text_layout: SolvedLayout,
	text: Text<T>,
	color: Lazy<Property<Color>>,
	on_click: Option<Box<dyn FnMut(f32, f32)>>,
	enabled: Lazy<bool>,
	is_mouse_down: bool,
	hovered: bool,
}

pub fn button<T: Display>(text: Text<T>) -> Button<T> {
	Button {
		layout: Layout::hug(),
		text_layout: SolvedLayout::default(),
		text,
		color: lazy(Property::new(Color::from_hex(0xff0051))),
		on_click: None,
		enabled: Lazy::new(true),
		is_mouse_down: false,
		hovered: false,
	}
}

impl<T: Display> Button<T> {
	pub fn with_layout(mut self, layout: Layout) -> Self {
		self.layout = layout;
		self
	}

	pub fn on_click(mut self, on_click: impl FnMut(f32, f32) + 'static) -> Self {
		self.on_click = Some(Box::new(on_click));
		self
	}
}

impl<T: Display> Widget for Button<T> {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		let layout = default_solve_layout(self, parent_layout);
		self.text_layout = (self.text).solve_layout(&layout.padded(Padding::vh(5., 10.)));
		layout
	}

	fn min_width(&self) -> f32 {
		self.text.min_width() + 10.
	}

	fn min_height(&self) -> f32 {
		self.text.min_height() + 5.
	}

	fn debug(&self, w: &mut dyn io::Write, deepness: usize) -> io::Result<()> {
		writeln!(
			w,
			"{}<button>{}</button>",
			indentation(deepness),
			self.text.text(),
		)
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

		let color = self.color.get_mut().current().into_skia();

		paint.set_stroke(false);
		paint.set_color(color.with_a(0x80));
		canvas.draw_rect(rect, &paint);

		paint.set_stroke(true);
		paint.set_color(color.with_a(0xff));
		canvas.draw_rect(rect, &paint);

		self.text.draw(canvas, &self.text_layout);
	}

	fn handle_event(&mut self, event: Event, _layout: &SolvedLayout) -> bool {
		if *self.enabled.get() {
			let handled = match event {
				Event::MouseDown(_) => {
					self.is_mouse_down = true;
					true
				}
				Event::MouseUp(MousePosition { x, y }) => {
					if self.is_mouse_down {
						self.is_mouse_down = false;
						if let Some(on_click) = self.on_click.as_mut() {
							(on_click)(x, y);
						}
						true
					} else {
						false
					}
				}
				Event::MouseIn => {
					self.hovered = true;
					true
				}
				Event::MouseOut => {
					self.hovered = false;
					self.is_mouse_down = false;
					true
				}
				_ => return false,
			};

			let color = match (*self.enabled.get(), self.is_mouse_down) {
				(true, true) => 0xa70038,
				(true, false) if self.hovered => 0x51ffff,
				(true, false) => 0xff0051,
				_ => 0x7a4553,
			};

			let color = Color::from_hex(color);
			(self.color.get_mut()).go_to(color, ease::out_quint, Duration::from_millis(500));

			handled
		} else {
			false
		}
	}
}
