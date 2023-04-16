use miniquad::skia::SkiaContext;
use skia_safe::{Color, Paint, Rect};

use crate::layout::{Layout, SolvedLayout};
use crate::lazy::Laz;
use crate::widget::{Event, Widget};

pub struct Button {
	layout: Layout,
	text: String,
	on_click: Option<Box<dyn FnMut(f32, f32)>>,
	enabled: Laz<bool>,
	hovered: bool,
}

impl Button {
	pub fn on_click(mut self, on_click: impl FnMut(f32, f32) + 'static) -> Self {
		self.on_click = Some(Box::new(on_click));
		self
	}
}

impl Widget for Button {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		self.default_solve_layout(parent_layout)
	}

	fn min_width(&mut self) -> f32 {
		self.text.len() as f32 * 10.
	}

	fn min_height(&mut self) -> f32 {
		15.
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
		paint.set_color(Color::from(0x80_ff0051));
		canvas.draw_rect(rect, &paint);

		paint.set_stroke(true);
		paint.set_color(Color::from(0xff_ff0051));
		canvas.draw_rect(rect, &paint);
	}

	fn handle_event(&mut self, event: Event) -> bool {
		if self.enabled.get() {
			match event {
				Event::Clicked(x, y) => {
					if let Some(on_click) = self.on_click.as_mut() {
						(on_click)(x, y);
					}
				}
				Event::HoverStart => {
					self.hovered = true;
				}
				Event::HoverEnd => {
					self.hovered = false;
				}
			}

			true
		} else {
			false
		}
	}
}

pub fn button(text: impl Into<String>) -> Button {
	Button {
		layout: Layout::new(),
		text: text.into(),
		on_click: None,
		enabled: Laz::new(true),
		hovered: false,
	}
}
