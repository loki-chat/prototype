use std::cell::{Cell, Ref, RefCell, RefMut};
use std::fmt::{self, Debug};
use std::ops::{Add, Deref, Mul, Sub};
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

	pub fn set(&self, val: T) {
		*self.0.borrow_mut() = val;
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

pub struct Laz<T: Copy>(Rc<Cell<T>>);

pub fn laz<T: Copy>(val: T) -> Laz<T> {
	Laz::new(val)
}

impl<T: Copy> Laz<T> {
	pub fn new(val: T) -> Self {
		Laz(Rc::new(Cell::new(val)))
	}
}

impl<T: Copy> Clone for Laz<T> {
	fn clone(&self) -> Self {
		Self(Rc::clone(&self.0))
	}
}

impl<T: Copy> Deref for Laz<T> {
	type Target = Cell<T>;

	fn deref(&self) -> &Self::Target {
		self.0.as_ref()
	}
}

impl<T: Copy + fmt::Display> fmt::Display for Laz<T> {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		self.get().fmt(f)
	}
}

#[derive(Clone, Copy)]
pub struct Color {
	r: f32,
	g: f32,
	b: f32,
}

impl Color {
	pub fn from_hex(hex: u32) -> Self {
		let r = ((hex >> 16) & 0xff) as u8;
		let g = ((hex >> 8) & 0xff) as u8;
		let b = (hex & 0xff) as u8;
		Self::from_rgb(r, g, b)
	}

	pub fn from_rgb(r: u8, g: u8, b: u8) -> Self {
		Self {
			r: r as f32 / 255.,
			g: g as f32 / 255.,
			b: b as f32 / 255.,
		}
	}

	pub fn rgb_f32(&self) -> (f32, f32, f32) {
		(
			(self.r * 255.).clamp(0., 255.).round(),
			(self.g * 255.).clamp(0., 255.).round(),
			(self.b * 255.).clamp(0., 255.).round(),
		)
	}

	pub fn rgb(&self) -> (u8, u8, u8) {
		let (r, g, b) = self.rgb_f32();
		(r as u8, g as u8, b as u8)
	}

	pub fn rgb_i32(&self) -> (i32, i32, i32) {
		let (r, g, b) = self.rgb_f32();
		(r as i32, g as i32, b as i32)
	}

	pub fn into_skia(self) -> skia_safe::Color {
		let (r, g, b) = self.rgb();
		skia_safe::Color::from_rgb(r, g, b)
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
		self.r += rhs.r;
		self.g += rhs.g;
		self.b += rhs.b;
		self
	}
}

impl Sub for Color {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self.r -= rhs.r;
		self.g -= rhs.g;
		self.b -= rhs.b;
		self
	}
}

impl Mul<f32> for Color {
	type Output = Self;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self.r *= rhs;
		self.g *= rhs;
		self.b *= rhs;
		self
	}
}