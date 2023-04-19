use std::fmt::Display;
use std::io;

use skia_safe::{Canvas, Color, Paint, Rect};

use crate::indentation;
use crate::layout::{Layout, Padding, SolvedLayout};
use crate::lazy::Laz;
use crate::widget::{default_solve_layout, Event, Widget};

use super::text::Text;

pub struct Button<T: Display> {
	layout: Layout,
	text_layout: SolvedLayout,
	text: Text<T>,
	on_click: Option<Box<dyn FnMut(f32, f32)>>,
	enabled: Laz<bool>,
	hovered: bool,
}

pub fn button<T: Display>(text: Text<T>) -> Button<T> {
	Button {
		layout: Layout::hug(),
		text_layout: SolvedLayout::default(),
		text,
		on_click: None,
		enabled: Laz::new(true),
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

		paint.set_stroke(false);
		paint.set_color(Color::from(0x80_ff0051));
		canvas.draw_rect(rect, &paint);

		paint.set_stroke(true);
		paint.set_color(Color::from(0xff_ff0051));
		canvas.draw_rect(rect, &paint);

		self.text.draw(canvas, &self.text_layout);
	}

	fn handle_event(&mut self, event: Event, layout: &SolvedLayout) -> bool {
		if self.enabled.get() {
			match event {
				Event::Clicked(x, y) => {
					if layout.contains(x, y) {
						if let Some(on_click) = self.on_click.as_mut() {
							(on_click)(x, y);
						}
						true
					} else {
						false
					}
				}
				Event::HoverStart => {
					self.hovered = true;
					true
				}
				Event::HoverEnd => {
					self.hovered = false;
					false
				}
			}
		} else {
			false
		}
	}
}
