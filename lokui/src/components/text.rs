use crate::layout::{SolvedLayout, Layout};
use crate::lazy::Lazy;
use crate::widget::{Event, Widget};

// pub fn text<T: AsRef<str>>(val: Lazy<T>) -> Text<T> {
// 	Text(val)
// }

pub struct Text<T: AsRef<str>> {
	layout: Layout,
	text: Lazy<T>,
}

impl<T: AsRef<str>> Widget for Text<T> {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn draw(&self, layout: SolvedLayout) {
		// TODO: draw text?
	}

	fn update(&mut self, _event: Event) -> bool {
		false
	}
}
