use std::{error::Error, ffi::CString, num::NonZeroU32};

use gl::types::*;
use glutin::{
	config::{ConfigTemplateBuilder, GlConfig},
	context::{
		ContextApi, ContextAttributesBuilder, NotCurrentGlContextSurfaceAccessor,
		PossiblyCurrentContext,
	},
	display::{GetGlDisplay, GlDisplay},
	surface::{Surface as GlutinSurface, SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use raw_window_handle::HasRawWindowHandle;
use skia_safe::{
	gpu::{gl::FramebufferInfo, BackendRenderTarget, SurfaceOrigin},
	ColorType, Surface,
};
use winit::{
	dpi::{PhysicalSize, Size},
	event_loop::EventLoop,
	window::{Window, WindowBuilder},
};

// Guarantee the drop order inside the FnMut closure. `Window` _must_ be dropped after
// `DirectContext`.
//
// https://github.com/rust-skia/rust-skia/issues/476
pub struct SkiaWindowCtx {
	pub fb_info: FramebufferInfo,
	pub num_samples: usize,
	pub stencil_size: usize,
	pub surface: Surface,
	pub gl_surface: GlutinSurface<WindowSurface>,
	pub gr_context: skia_safe::gpu::DirectContext,
	pub gl_context: PossiblyCurrentContext,
	pub window: Window,
	pub events: EventLoop<()>,
}

pub fn create_skia_window(
	title: &str,
	width: u32,
	height: u32,
) -> Result<SkiaWindowCtx, Box<dyn Error>> {
	let events = EventLoop::new();

	let winit_window_builder = WindowBuilder::new()
		.with_title(title)
		.with_inner_size(Size::Physical(PhysicalSize::new(width, height)));

	let template = ConfigTemplateBuilder::new()
		.with_alpha_size(8)
		.with_transparency(true);

	let display_builder = DisplayBuilder::new().with_window_builder(Some(winit_window_builder));
	let (window, gl_config) = display_builder
		.build(&events, template, |configs| {
			// Find the config with the minimum number of samples. Usually Skia takes care of
			// anti-aliasing and may not be able to create appropriate Surfaces for samples > 0.
			// See https://github.com/rust-skia/rust-skia/issues/782
			// And https://github.com/rust-skia/rust-skia/issues/764
			configs
				.reduce(|accum, config| {
					let transparency_check = config.supports_transparency().unwrap_or(false)
						& !accum.supports_transparency().unwrap_or(false);

					if transparency_check || config.num_samples() < accum.num_samples() {
						config
					} else {
						accum
					}
				})
				.unwrap()
		})
		.unwrap();

	println!("Picked a config with {} samples", gl_config.num_samples());

	let mut window = window.expect("Could not create window with OpenGL context");
	let raw_window_handle = window.raw_window_handle();

	// The context creation part. It can be created before surface and that's how
	// it's expected in multithreaded + multiwindow operation mode, since you
	// can send NotCurrentContext, but not Surface.
	let context_attributes = ContextAttributesBuilder::new().build(Some(raw_window_handle));

	// Since glutin by default tries to create OpenGL core context, which may not be
	// present we should try gles.
	let fallback_context_attributes = ContextAttributesBuilder::new()
		.with_context_api(ContextApi::Gles(None))
		.build(Some(raw_window_handle));

	let not_current_gl_context = unsafe {
		gl_config
			.display()
			.create_context(&gl_config, &context_attributes)
			.unwrap_or_else(|_| {
				gl_config
					.display()
					.create_context(&gl_config, &fallback_context_attributes)
					.expect("failed to create context")
			})
	};

	let attrs = SurfaceAttributesBuilder::<WindowSurface>::new().build(
		raw_window_handle,
		NonZeroU32::new(width).unwrap(),
		NonZeroU32::new(height).unwrap(),
	);

	let gl_surface = unsafe {
		gl_config
			.display()
			.create_window_surface(&gl_config, &attrs)
			.expect("Could not create gl window surface")
	};

	let gl_context = not_current_gl_context
		.make_current(&gl_surface)
		.expect("Could not make GL context current when setting up skia renderer");

	gl::load_with(|s| {
		gl_config
			.display()
			.get_proc_address(CString::new(s).unwrap().as_c_str())
	});

	let interface = skia_safe::gpu::gl::Interface::new_load_with(|name| {
		if name == "eglGetCurrentDisplay" {
			return std::ptr::null();
		}
		gl_config
			.display()
			.get_proc_address(CString::new(name).unwrap().as_c_str())
	})
	.expect("Could not create interface");

	let mut gr_context = skia_safe::gpu::DirectContext::new_gl(Some(interface), None)
		.expect("Could not create direct context");

	let fb_info = {
		let mut fboid: GLint = 0;
		unsafe { gl::GetIntegerv(gl::FRAMEBUFFER_BINDING, &mut fboid) };

		FramebufferInfo {
			fboid: fboid.try_into().unwrap(),
			format: skia_safe::gpu::gl::Format::RGBA8.into(),
		}
	};

	let num_samples = gl_config.num_samples() as usize;
	let stencil_size = gl_config.stencil_size() as usize;

	let surface = create_surface(
		&mut window,
		fb_info,
		&mut gr_context,
		num_samples,
		stencil_size,
	);

	Ok(SkiaWindowCtx {
		fb_info,
		num_samples,
		stencil_size,
		surface,
		gl_surface,
		gr_context,
		gl_context,
		window,
		events,
	})
}

pub fn create_surface(
	window: &mut Window,
	fb_info: FramebufferInfo,
	gr_context: &mut skia_safe::gpu::DirectContext,
	num_samples: usize,
	stencil_size: usize,
) -> Surface {
	let size = window.inner_size();
	let size = (size.width as i32, size.height as i32);

	let backend_render_target =
		BackendRenderTarget::new_gl(size, num_samples, stencil_size, fb_info);

	Surface::from_backend_render_target(
		gr_context,
		&backend_render_target,
		SurfaceOrigin::BottomLeft,
		ColorType::RGBA8888,
		None,
		None,
	)
	.expect("Could not create skia surface")
}
