#![allow(clippy::unusual_byte_groupings)]

use std::io::{stdout, BufWriter, Write};

use lokui::components::button::button;
use lokui::components::pane::pane;
use lokui::components::text::text;
use lokui::components::WidgetExt;
use lokui::events::{Event, MousePosition};
use lokui::layout::{Anchor, DimScalar, Direction, FlexLayout, Layout, Padding, SolvedLayout};
use lokui::state::{lazy, Color, RectState};
use lokui::widget::{Widget, WidgetContainer};
use miniquad::skia::SkiaContext;
use miniquad::{conf, EventHandler};
use skia_safe::{Font, FontStyle, Typeface};

fn counter() -> impl Widget {
	let value = lazy(0);

	let increment = {
		let value = value.clone();
		move |_, _| {
			*value.get_mut() += 1;
			println!("+1! Counter = {}", value.get());
		}
	};

	let decrement = {
		let value = value.clone();
		move |_, _| {
			*value.get_mut() -= 1;
			println!("-1! Counter = {}", value.get());
		}
	};

	let typeface = Typeface::new("Torus Pro", FontStyle::normal()).unwrap();
	let font = lazy(Font::new(typeface, Some(20.)));

	pane()
		.with_layout(
			Layout::new()
				.with_anchor(Anchor::CENTER)
				.with_dimension(DimScalar::Fixed(400.), DimScalar::Fixed(250.)),
		)
		.padding(Padding::splat(10.))
		.bg(lazy(RectState::new(
			Color::from_hex(0xff_2e428c),
			10.,
			None,
		)))
		.child(
			pane()
				.with_flex_layout(FlexLayout {
					direction: Direction::Horizontal,
					gap: 5.,
				})
				.with_layout(Layout::new().with_dimension(DimScalar::Fill, DimScalar::Fill))
				.padding(Padding::vh(5., 10.))
				.bg(lazy(RectState::new(Color::from_hex(0x80_657cb1), 5., None)))
				.child(
					pane()
						.with_layout(
							Layout::new()
								.with_dimension(DimScalar::Fill, DimScalar::Fixed(50.))
								.with_origin(Anchor::CENTER)
								.with_anchor(Anchor::CENTER),
						)
						.child(text(value, font.clone()))
						.bg(lazy(RectState::new(Color::from_hex(0xff_33aa55), 5., None))),
				)
				.child(
					button(text("+1", font.clone()))
						.with_layout(
							Layout::new()
								.with_dimension(DimScalar::Fixed(80.), DimScalar::Fixed(50.))
								.with_origin(Anchor::CENTER)
								.with_anchor(Anchor::CENTER),
						)
						.on_click(increment),
				)
				.child(
					button(text("-1", font))
						.with_layout(
							Layout::new()
								.with_dimension(DimScalar::Fixed(80.), DimScalar::Fixed(50.))
								.with_origin(Anchor::CENTER)
								.with_anchor(Anchor::CENTER),
						)
						.on_click(decrement),
				),
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
