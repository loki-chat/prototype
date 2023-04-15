use std::ops::Deref;

use lokui::components::button::button;
use lokui::components::number::number;
use lokui::components::pane::{pane, Pane};
use lokui::components::text::text;
use lokui::lazy::{Laz, Lazy};
use lokui::widget::{Event, Widget};

struct Counter {
	value: Laz<i64>,
	inner: Pane,
}

impl Counter {
	fn new() -> Self {
		let value = Laz::new(0);

		let increment = {
			let value = value.clone();
			move || value.set(value.get() + 1)
		};

		let decrement = {
			let value = value.clone();
			move || value.set(value.get() - 1)
		};

		let inner = pane()
			.child(text(Lazy::new("Count: ")))
			.child(number(value.clone()))
			.child(
				pane()
					.child(button("+1").on_click(increment))
					.child(button("-1").on_click(decrement)),
			);

		Counter { value, inner }
	}

	fn value(&self) -> &Laz<i64> {
		&self.value
	}
}

impl Deref for Counter {
	type Target = Pane;

	fn deref(&self) -> &Self::Target {
		&self.inner
	}
}

impl Widget for Counter {
	fn draw(&self, indent: usize) {
		self.inner.draw(indent);
	}

	fn update(&mut self, event: Event) -> bool {
		self.inner.update(event)
	}
}

fn main() {
	let mut counter = Counter::new();

	counter.draw(0);
	println!("\nThe value of the counter is {}\n", counter.value().get());

	counter.update(Event::Clicked);
	counter.draw(0);
	println!("\nThe value of the counter is {}\n", counter.value().get());
}
