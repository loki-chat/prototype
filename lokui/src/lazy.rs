use std::cell::{Cell, Ref, RefCell};
use std::ops::Deref;
use std::rc::Rc;

pub struct Lazy<T>(Rc<RefCell<T>>);

impl<T> Lazy<T> {
	pub fn new(val: T) -> Self {
		Lazy(Rc::new(RefCell::new(val)))
	}

	pub fn get(&self) -> Ref<T> {
		self.0.borrow()
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

pub struct Laz<T: Copy>(Rc<Cell<T>>);

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
