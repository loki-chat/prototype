#![allow(clippy::unusual_byte_groupings)]

use std::io::{stdout, BufWriter, Write};
use std::time::Duration;

use lokui::anim::ease;
use lokui::events::{Event, MousePosition};
use lokui::layout::SolvedLayout;
use lokui::prelude::*;

use lokui::state::Color;
use miniquad::skia::SkiaContext;
use miniquad::{conf, EventHandler};
use skia_safe::{Font, FontStyle, Typeface};

/// Curried increment callback.
fn increment(value: Lazy<i32>) -> impl Fn(f32, f32) -> bool {
	move |_, _| {
		*value.get_mut() += 1;
		println!("+1! Counter = {}", value.get());
		true
	}
}

/// Curried decrement callback.
fn decrement(value: Lazy<i32>) -> impl Fn(f32, f32) -> bool {
	move |_, _| {
		*value.get_mut() -= 1;
		println!("-1! Counter = {}", value.get());
		true
	}
}

fn counter_button_color_handler(bg: Lazy<RectState>) -> impl FnMut(Event) -> bool {
	/// wrapper struct to have a simple non-copy background color
	struct BgColor(u32);

	let mut bg_color = BgColor(0xff0051);
	move |event| {
		bg_color.0 = match event {
			Event::MouseDown(_) => 0xa70038,
			Event::MouseIn | Event::MouseUp(_) => 0x51ffff,
			Event::MouseOut => 0xff0051,
			_ => bg_color.0,
		};

		let color = Color::from_hex(0xff_000000 | bg_color.0);
		(bg.get_mut().color).go_to(color, ease::out_quint, Duration::from_millis(500));

		false
	}
}

fn counter_button(
	text: impl Widget + 'static,
	on_click: impl FnMut(f32, f32) -> bool + 'static,
) -> impl Widget {
	let background = lazy(RectState::new(0xff_ff0051, 5., None));

	pane()
		.with_layout(
			Layout::new()
				.with_dimension(Fixed(80.), Fixed(50.))
				.with_origin(Anchor::CENTER)
				.with_anchor(Anchor::CENTER),
		)
		.child(text)
		.bg(background.clone())
		.on_click(on_click)
		.on_event(counter_button_color_handler(background))
}

fn counter() -> impl Widget {
	let value = lazy(0);

	let typeface = Typeface::new("Roboto", FontStyle::normal()).unwrap();
	let font = lazy(Font::new(typeface, Some(20.)));

	pane()
		.with_layout(
			Layout::new()
				.with_anchor(Anchor::CENTER)
				.with_dimension(Fixed(400.), Fixed(250.)),
		)
		.with_padding(Padding::splat(10.))
		.bg(lazy(RectState::new(0xff_2e428c, 10., None)))
		.child(
			pane()
				.with_flex(Flex {
					direction: Direction::Horizontal,
					gap: 5.,
				})
				.with_layout(Layout::new().with_dimension(Fill, Fill))
				.with_padding(Padding::vh(5., 10.))
				.bg(lazy(RectState::new(0x80_657cb1, 5., None)))
				.child(
					pane()
						.with_layout(
							Layout::new()
								.with_dimension(Fill, Fixed(50.))
								.with_origin(Anchor::CENTER)
								.with_anchor(Anchor::CENTER),
						)
						.child(text(value.clone(), font.clone()))
						.bg(lazy(RectState::new(0xff_33aa55, 5., None))),
				)
				.child(counter_button(
					text("+1", font.clone()),
					increment(value.clone()),
				))
				.child(counter_button(
					text("-1", font),
					decrement(value),
				)),
		)
}

struct Stage<W: Widget> {
	root_widget: W,
	root_layout: SolvedLayout,
	window_layout: SolvedLayout,
}

impl<W: Widget> EventHandler for Stage<W> {
	fn update(&mut self, _skia_ctx: &mut SkiaContext) {}

	fn mouse_button_down_event(
		&mut self,
		_skia_ctx: &mut SkiaContext,
		_button: miniquad::MouseButton,
		x: f32,
		y: f32,
	) {
		let event = Event::MouseDown(MousePosition { x, y });
		self.root_widget.handle_event(event, &self.root_layout);
	}

	fn mouse_button_up_event(
		&mut self,
		_skia_ctx: &mut SkiaContext,
		_button: miniquad::MouseButton,
		x: f32,
		y: f32,
	) {
		let event = Event::MouseUp(MousePosition { x, y });
		self.root_widget.handle_event(event, &self.root_layout);
	}

	fn mouse_motion_event(&mut self, _skia_ctx: &mut SkiaContext, x: f32, y: f32) {
		let event = Event::MouseMove(MousePosition { x, y });
		self.root_widget.handle_event(event, &self.root_layout);
	}

	fn resize_event(&mut self, skia_ctx: &mut SkiaContext, width: f32, height: f32) {
		self.window_layout = SolvedLayout::from_top_left(0., 0., width, height);
		self.root_layout = self.root_widget.solve_layout(&self.window_layout);
		skia_ctx.recreate_surface(width as i32, height as i32);
	}

	fn draw(&mut self, skia_ctx: &mut SkiaContext) {
		let canvas = skia_ctx.surface.canvas();
		canvas.clear(skia_safe::Color::from(0xff_161a1d));
		self.root_widget.draw(canvas, &self.root_layout);
		skia_ctx.dctx.flush(None);
	}
}

fn main() {
	let mut root_pane = counter();
	let window_layout = SolvedLayout::from_top_left(0., 0., 1280., 720.);
	let root_layout = root_pane.solve_layout(&window_layout);

	let mut writer = BufWriter::new(stdout());
	root_pane.debug(&mut writer, 0).unwrap();
	writer.flush().unwrap();

	miniquad::start(
		conf::Conf {
			high_dpi: true,
			window_width: 1280,
			window_height: 720,
			window_title: "Lokui GUI Framework Prototype".to_owned(),
			..Default::default()
		},
		move || {
			Box::new(Stage {
				root_widget: root_pane,
				root_layout,
				window_layout,
			})
		},
	);
}
