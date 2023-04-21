use std::fmt::Debug;
use std::ops::{Add, Mul, Sub};
use std::time::{Duration, Instant};

pub mod ease;

impl<I: Add<Output = I> + Sub<Output = I> + Mul<f32, Output = I> + Copy> Interpolate for I {}
pub trait Interpolate:
	Add<Output = Self> + Sub<Output = Self> + Mul<f32, Output = Self> + Copy
{
}

pub fn lerp<I: Interpolate>(t: f32, start: I, end: I) -> I {
	start + (end - start) * t
}

pub fn interp<I: Interpolate>(ease_fn: fn(f32) -> f32, t: f32, start: I, end: I) -> I {
	lerp(ease_fn(t), start, end)
}

#[derive(Clone)]
pub enum Property<T: Interpolate> {
	Static {
		value: T,
	},
	Anim {
		prev: T,
		next: T,
		start: Instant,
		ease_fn: fn(f32) -> f32,
		duration: Duration,
	},
}

impl<T: Interpolate> Property<T> {
	pub fn new(value: T) -> Self {
		Self::Static { value }
	}

	pub fn go_to(&mut self, next: T, ease_fn: fn(f32) -> f32, duration: Duration) {
		let prev = self.current();
		let start = Instant::now();
		*self = Self::Anim {
			prev,
			next,
			start,
			ease_fn,
			duration,
		};
	}

	pub fn set(&mut self, value: T) {
		*self = Self::Static { value };
	}

	pub fn current(&mut self) -> T {
		match self {
			Self::Static { value } => *value,
			Self::Anim {
				prev,
				next,
				start,
				ease_fn,
				duration,
			} => {
				let t = (start.elapsed().as_secs_f32() / duration.as_secs_f32()).clamp(0., 1.);
				let value = interp(*ease_fn, t, *prev, *next);
				if t == 1. {
					// make static
					self.set(value);
				}
				value
			}
		}
	}

	pub fn freeze(&mut self) {
		if self.is_anim() {
			let value = self.current();
			self.set(value);
		}
	}

	pub fn is_static(&self) -> bool {
		matches!(self, Self::Static { .. })
	}

	pub fn is_anim(&self) -> bool {
		matches!(self, Self::Anim { .. })
	}
}

impl<T: Interpolate + Debug> Debug for Property<T> {
	fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
		match self {
			Self::Static { value } => write!(f, "static ({value:?})"),
			Self::Anim {
				prev,
				next,
				duration,
				..
			} => {
				write!(
					f,
					"anim [{}ms] ({prev:?} -> {next:?})",
					duration.as_millis()
				)
			}
		}
	}
}
