use crate::lazy::Lazy;
use crate::widget::{Event, Widget};

pub fn text<T: AsRef<str>>(val: Lazy<T>) -> Text<T> {
	Text(val)
}

pub struct Text<T: AsRef<str>>(Lazy<T>);

impl<T: AsRef<str>> Widget for Text<T> {
	fn draw(&self, indent: usize) {
		println!(
			"{}<text>{}</text>",
			"  ".repeat(indent),
			self.0.get().as_ref()
		);
	}

	fn update(&mut self, _event: Event) -> bool {
		false
	}
}
