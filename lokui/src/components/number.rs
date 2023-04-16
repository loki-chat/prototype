use crate::layout::{Layout, SolvedLayout};
use crate::lazy::Laz;
use crate::widget::{Event, Widget};

pub struct Number {
	layout: Layout,
	number: Laz<i64>,
}

// pub fn number(val: Laz<i64>) -> Number {
// 	Number(val)
// }

impl Widget for Number {
	fn layout(&self) -> &Layout {
		&self.layout
	}

	fn draw(&self, layout: SolvedLayout) {
		// TODO: draw number as text?
	}

	fn update(&mut self, _event: Event) -> bool {
		false
	}
}
