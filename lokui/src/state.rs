use std::cell::{Ref, RefCell, RefMut};
use std::fmt::{self, Debug};
use std::ops::{Add, Mul, Sub};
use std::rc::Rc;

pub struct Lazy<T>(Rc<RefCell<T>>);

pub fn lazy<T>(val: T) -> Lazy<T> {
	Lazy::new(val)
}

impl<T> Lazy<T> {
	pub fn new(val: T) -> Self {
		Lazy(Rc::new(RefCell::new(val)))
	}

	pub fn get(&self) -> Ref<T> {
		(*self.0).borrow()
	}

	pub fn get_mut(&self) -> RefMut<T> {
		(*self.0).borrow_mut()
	}
}

impl<T> Clone for Lazy<T> {
	fn clone(&self) -> Self {
		Self(Rc::clone(&self.0))
	}
}

impl<T: Copy + fmt::Display> fmt::Display for Lazy<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.get().fmt(f)
	}
}

#[derive(Clone, Copy)]
pub struct Color {
	a: f32,
	r: f32,
	g: f32,
	b: f32,
}

impl Color {
	pub fn from_hex(hex: u32) -> Self {
		let a = ((hex >> 24) & 0xff) as u8;
		let r = ((hex >> 16) & 0xff) as u8;
		let g = ((hex >> 8) & 0xff) as u8;
		let b = (hex & 0xff) as u8;
		Self::from_argb(a, r, g, b)
	}

	pub fn from_argb(a: u8, r: u8, g: u8, b: u8) -> Self {
		Self {
			a: a as f32 / 255.,
			r: r as f32 / 255.,
			g: g as f32 / 255.,
			b: b as f32 / 255.,
		}
	}

	pub fn argb_f32(&self) -> (f32, f32, f32, f32) {
		(
			(self.a * 255.).clamp(0., 255.).round(),
			(self.r * 255.).clamp(0., 255.).round(),
			(self.g * 255.).clamp(0., 255.).round(),
			(self.b * 255.).clamp(0., 255.).round(),
		)
	}

	pub fn argb(&self) -> (u8, u8, u8, u8) {
		let (a, r, g, b) = self.argb_f32();
		(a as u8, r as u8, g as u8, b as u8)
	}

	pub fn rgb_i32(&self) -> (i32, i32, i32, i32) {
		let (a, r, g, b) = self.argb_f32();
		(a as i32, r as i32, g as i32, b as i32)
	}

	pub fn into_skia(self) -> skia_safe::Color {
		let (a, r, g, b) = self.argb();
		skia_safe::Color::from_argb(a, r, g, b)
	}
}

impl Debug for Color {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.rgb_i32().fmt(f)
	}
}

impl Add for Color {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self.a += rhs.a;
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
		self
	}
}

impl Sub for Color {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.a -= rhs.a;
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
		self
	}
}

impl Mul<f32> for Color {
	type Output = Self;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.a *= rhs;
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
		self
	}
}
