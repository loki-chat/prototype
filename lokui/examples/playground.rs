#![allow(clippy::unusual_byte_groupings)]

use std::ops::Deref;

use lokui::components::button::button;
// use lokui::components::number::number;
use lokui::components::pane::{pane, Pane};
use lokui::layout::{Anchor, DimScalar, Layout, Padding, SolvedLayout};
// use lokui::components::text::text;
use lokui::lazy::Laz;
use lokui::widget::{Event, Widget};
use miniquad::skia::SkiaContext;
use miniquad::{conf, EventHandler};
use skia_safe::Color;

struct Counter {
	value: Laz<i64>,
	layout: Layout,
	inner: Pane,
}

impl Counter {
	fn new() -> Self {
		let value = Laz::new(0);

		let increment = {
			let value = value.clone();
			move |_, _| value.set(value.get() + 1)
		};

		// let decrement = {
		// 	let value = value.clone();
		// 	move || value.set(value.get() - 1)
		// };

		let inner = pane()
			.with_padding(Padding::splat(10.))
			.with_layout(
				Layout::new()
					.with_anchor(Anchor::CENTER)
					.with_dimension(DimScalar::Fixed(400.), DimScalar::Fixed(250.)),
			)
			// .child(text(Lazy::new("Count: ")))
			// .child(number(value.clone()))
			.child(
				pane()
					.with_padding(Padding::vh(5., 30.))
					.with_layout(Layout::new().with_dimension(DimScalar::Fill, DimScalar::Fill))
					.child(button("+1").on_click(increment)),
				// .child(button("-1").on_click(decrement)),
			);

		Counter {
			value,
			inner,
			layout: Layout::new(),
		}
	}

	fn value(&self) -> &Laz<i64> {
		&self.value
	}
}

impl Deref for Counter {
	type Target = Pane;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Widget for Counter {
	fn handle_event(&mut self, event: Event) -> bool {
		self.inner.handle_event(event)
	}

	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn solve_layout(&mut self, parent_layout: &SolvedLayout) -> SolvedLayout {
		self.inner.solve_layout(parent_layout)
	}

	fn draw(&self, skia_ctx: &mut SkiaContext, layout: &SolvedLayout) {
		self.inner.draw(skia_ctx, layout);
	}
}

struct Stage {
	counter: Counter,
	window_layout: SolvedLayout,
	counter_layout: SolvedLayout,
}

impl EventHandler for Stage {
	fn update(&mut self, _skia_ctx: &mut SkiaContext) {}

	fn draw(&mut self, skia_ctx: &mut SkiaContext) {
		let canvas = &mut skia_ctx.surface.canvas();
		canvas.clear(Color::from(0xff_161a1d));

		self.counter.draw(skia_ctx, &self.counter_layout);

		skia_ctx.dctx.flush(None);
	}
}

fn main() {
	let mut counter = Counter::new();
	let window_layout =
		SolvedLayout::from_top_left(0., 0., 1280., 720.).padded(Padding::splat(20.));
	let counter_layout = counter.solve_layout(&window_layout);

	miniquad::start(
		conf::Conf {
			high_dpi: true,
			window_width: 1280,
			window_height: 720,
			window_resizable: false,
			window_title: "Lokui GUI Framework Prototype".to_owned(),
			..Default::default()
		},
		move || {
			Box::new(Stage {
				counter,
				window_layout,
				counter_layout,
			})
		},
	);
}
