use std::fmt::Display;
use std::io;

use miniquad::skia::SkiaContext;
use skia_safe::{Color, Font, Paint, Rect};

use crate::indentation;
use crate::layout::{Anchor, Layout, SolvedLayout};
use crate::lazy::Lazy;
use crate::widget::{Event, Widget};

pub struct Text<T: Display> {
	layout: Layout,
	font: Lazy<Font>,
	text: T,
	// min_bounds: Rect,
}

pub fn text<T: Display>(text: T, font: Lazy<Font>) -> Text<T> {
	Text {
		layout: Layout::hug(),
		font,
		text,
		// min_bounds: Rect::default(),
	}
}

impl<T: Display> Text<T> {
	pub fn with_layout(mut self, layout: Layout) -> Self {
		self.layout = layout;
		self
	}

	pub fn text(&self) -> &T {
		&self.text
	}

	pub fn min_bounds(&self) -> Rect {
		(self.font.get())
			.measure_str(format!("{}", &self.text), None)
			.1
	}
}

impl<T: Display> Widget for Text<T> {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		self.default_solve_layout(parent_layout)
	}

	fn min_width(&self) -> f32 {
		self.min_bounds().width()
	}

	fn min_height(&self) -> f32 {
		self.min_bounds().height()
	}

	fn debug(&self, w: &mut dyn io::Write, deepness: usize) -> io::Result<()> {
		writeln!(
			w,
			"{}<text>{}</text>",
			indentation(deepness),
			&self.text,
		)
	}

	fn draw(&self, skia_ctx: &mut SkiaContext, layout: &SolvedLayout) {
		let canvas = skia_ctx.surface.canvas();

		let mut paint = Paint::default();
		paint.set_anti_alias(true);
		paint.set_color(Color::from(0xff_ffffff));

		let (x, y) = layout.point_at_anchor(Anchor::TOP_LEFT);

		canvas.draw_str(
			format!("{}", &self.text),
			(x, y + self.min_bounds().height()),
			self.font.get().as_ref(),
			&paint,
		);
	}

	fn handle_event(&mut self, _event: Event, _layout: &SolvedLayout) -> bool {
		false
	}
}
