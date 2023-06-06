use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

#[derive(Clone, Copy, Debug, Default)]
pub struct Padding {
	pub top: f32,
	pub right: f32,
	pub bottom: f32,
	pub left: f32,
}

impl Padding {
	pub fn trbl(top: f32, right: f32, bottom: f32, left: f32) -> Self {
		Self {
			top,
			right,
			bottom,
			left,
		}
	}

	pub fn vh(vertical: f32, horizontal: f32) -> Self {
		Self {
			top: vertical,
			right: horizontal,
			bottom: vertical,
			left: horizontal,
		}
	}

	pub fn splat(val: f32) -> Self {
		Self {
			top: val,
			right: val,
			bottom: val,
			left: val,
		}
	}
}

// add

impl Add for Padding {
	type Output = Self;

	fn add(mut self, rhs: Self) -> Self::Output {
		self += rhs;
		self
	}
}

impl Add<f32> for Padding {
	type Output = Self;

	fn add(mut self, rhs: f32) -> Self::Output {
		self += rhs;
		self
	}
}

impl AddAssign for Padding {
	fn add_assign(&mut self, rhs: Self) {
		self.top += rhs.top;
		self.right += rhs.right;
		self.bottom += rhs.bottom;
		self.left += rhs.left;
	}
}

impl AddAssign<f32> for Padding {
	fn add_assign(&mut self, rhs: f32) {
		self.top += rhs;
		self.right += rhs;
		self.bottom += rhs;
		self.left += rhs;
	}
}

// sub

impl Sub for Padding {
	type Output = Self;

	fn sub(mut self, rhs: Self) -> Self::Output {
		self -= rhs;
		self
	}
}

impl Sub<f32> for Padding {
	type Output = Self;

	fn sub(mut self, rhs: f32) -> Self::Output {
		self -= rhs;
		self
	}
}

impl SubAssign for Padding {
	fn sub_assign(&mut self, rhs: Self) {
		self.top -= rhs.top;
		self.right -= rhs.right;
		self.bottom -= rhs.bottom;
		self.left -= rhs.left;
	}
}

impl SubAssign<f32> for Padding {
	fn sub_assign(&mut self, rhs: f32) {
		self.top -= rhs;
		self.right -= rhs;
		self.bottom -= rhs;
		self.left -= rhs;
	}
}

// mul

impl Mul<f32> for Padding {
	type Output = Self;

	fn mul(mut self, rhs: f32) -> Self::Output {
		self *= rhs;
		self
	}
}

impl MulAssign<f32> for Padding {
	fn mul_assign(&mut self, rhs: f32) {
		self.top *= rhs;
		self.right *= rhs;
		self.bottom *= rhs;
		self.left *= rhs;
	}
}

// div

impl Div<f32> for Padding {
	type Output = Self;

	fn div(mut self, rhs: f32) -> Self::Output {
		self /= rhs;
		self
	}
}

impl DivAssign<f32> for Padding {
	fn div_assign(&mut self, rhs: f32) {
		self.top /= rhs;
		self.right /= rhs;
		self.bottom /= rhs;
		self.left /= rhs;
	}
}
