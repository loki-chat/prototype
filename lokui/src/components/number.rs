use crate::lazy::Laz;
use crate::widget::{Widget, Event};

pub struct Number(Laz<i64>);

pub fn number(val: Laz<i64>) -> Number {
	Number(val)
}

impl Widget for Number {
	fn draw(&self, indent: usize) {
		println!("{}<num>{}</num>", "  ".repeat(indent), self.0.get());
	}

	fn update(&mut self, _event: Event) -> bool {
		false
	}
}
