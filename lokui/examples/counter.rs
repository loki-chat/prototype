#![allow(clippy::unusual_byte_groupings)]

use std::num::NonZeroU32;
use std::time::Duration;

use glutin::surface::GlSurface;
use lokui::anim::ease;
use lokui::events::MousePosition;
use lokui::layout::SolvedLayout;
use lokui::prelude::*;

use lokui::events::Event as LokuiEvent;
use winit::event::{ElementState, Event, MouseButton};

use lokui::state::Color;
use skia_safe::{Font, FontStyle, Typeface};
use skia_window::{create_skia_window, SkiaWindowCtx};
use winit::event::{KeyboardInput, VirtualKeyCode, WindowEvent};

use crate::skia_window::create_surface;

#[path = "common/skia_window.rs"]
mod skia_window;

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

fn counter_button_color_handler(bg: Lazy<RectState>) -> impl FnMut(LokuiEvent) -> bool {
	/// wrapper struct to have a simple non-copy background color
	struct BgColor(u32);

	let idle_color = bg.get_mut().color.current().argb_hex();

	let mut bg_color = BgColor(idle_color);
	move |event| {
		bg_color.0 = match event {
			LokuiEvent::MouseDown(_) => 0x3d3556,
			LokuiEvent::MouseIn | LokuiEvent::MouseUp(_) => 0xaa95f0,
			LokuiEvent::MouseOut => idle_color,
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
	let background = lazy(RectState::new(0xff_000000 | 0x8460f0, 5., None));

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
	let font = lazy(Font::new(typeface, Some(16.)));

	pane()
		.with_layout(
			Layout::new()
				.with_anchor(Anchor::CENTER)
				.with_dimension(Fixed(400.), Fixed(250.)),
		)
		.with_padding(Padding::splat(10.))
		.bg(lazy(RectState::new(0xff_232128, 10., None)))
		.child(
			pane()
				.with_flex(Flex {
					direction: Direction::Horizontal,
					gap: 5.,
				})
				.with_layout(Layout::new().with_dimension(Fill, Fill))
				.with_padding(Padding::vh(5., 10.))
				.bg(lazy(RectState::new(0xff_2e2c35, 5., None)))
				.child(
					pane()
						.with_layout(
							Layout::new()
								.with_dimension(Fill, Fixed(50.))
								.with_origin(Anchor::CENTER)
								.with_anchor(Anchor::CENTER),
						)
						.child(text(value.clone(), font.clone()))
						.bg(lazy(RectState::new(0xff_232128, 5., None))),
				)
				.child(counter_button(
					text("+1", font.clone()),
					increment(value.clone()),
				))
				.child(counter_button(text("-1", font), decrement(value))),
		)
}

struct RootWidgetTree<W: Widget> {
	root_widget: W,
	root_layout: SolvedLayout,
	window_layout: SolvedLayout,
}

fn main() {
	let win = create_skia_window("Lokui GUI Framework Prototype (Counter)", 1280, 720).unwrap();

	let mut rwt = {
		let mut root_widget = counter();
		let window_layout = SolvedLayout::from_top_left(0., 0., 1280., 720.);
		let root_layout = root_widget.solve_layout(&window_layout);

		RootWidgetTree {
			root_widget,
			root_layout,
			window_layout,
		}
	};

	let mut x = 0;
	let mut y = 0;

	let SkiaWindowCtx {
		fb_info,
		num_samples,
		stencil_size,
		mut surface,
		gl_surface,
		mut gr_context,
		gl_context,
		mut window,
		events,
	} = win;

	window.request_redraw();
	events.run(move |event, _, control_flow| {
		let window = &mut window;
		let surface = &mut surface;
		let gr_context = &mut gr_context;
		let gl_context = &gl_context;

		match event {
			Event::LoopDestroyed => {}
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::CloseRequested => {
					println!("Closing...");
					control_flow.set_exit();
				}
				WindowEvent::Resized(physical_size) => {
					/* First resize the opengl drawable */
					let (width, height): (u32, u32) = physical_size.into();

					rwt.window_layout =
						SolvedLayout::from_top_left(0., 0., width as f32, height as f32);
					rwt.root_layout = rwt.root_widget.solve_layout(&rwt.window_layout);

					*surface = create_surface(
						window,
						fb_info,
						gr_context,
						num_samples,
						stencil_size,
					);

					gl_surface.resize(
						gl_context,
						NonZeroU32::new(width.max(1)).unwrap(),
						NonZeroU32::new(height.max(1)).unwrap(),
					);

					window.request_redraw();
					control_flow.set_wait();
				}
				WindowEvent::MouseInput { state, button, .. } => {
					if button == MouseButton::Left {
						let mouse_pos = MousePosition {
							x: x as f32,
							y: y as f32,
						};

						let event = match state {
							ElementState::Pressed => LokuiEvent::MouseDown(mouse_pos),
							ElementState::Released => LokuiEvent::MouseUp(mouse_pos),
						};

						rwt.root_widget.handle_event(event, &rwt.root_layout);
					}

					control_flow.set_wait();
				}
				WindowEvent::CursorMoved { position, .. } => {
					x = position.x as u32;
					y = position.y as u32;

					let event = LokuiEvent::MouseMove(MousePosition {
						x: position.x as f32,
						y: position.y as f32,
					});
					rwt.root_widget.handle_event(event, &rwt.root_layout);

					control_flow.set_wait();
				}
				WindowEvent::KeyboardInput {
					input:
						KeyboardInput {
							virtual_keycode: Some(VirtualKeyCode::Q),
							..
						},
					..
				} => control_flow.set_exit(),
				_ => control_flow.set_wait(),
			},
			Event::RedrawRequested(_) => {
				let canvas = surface.canvas();
				canvas.clear(skia_safe::Color::from(0xff_161a1d));
				rwt.root_widget.draw(canvas, &rwt.root_layout);

				gr_context.flush_and_submit();
				gl_surface.swap_buffers(gl_context).unwrap();

				window.request_redraw();
				control_flow.set_wait();
			}
			_ => control_flow.set_wait(),
		}
	});
}
