use crate::widget::{Event, Widget};

#[derive(Default)]
pub struct Pane {
	children: Vec<Box<dyn Widget>>,
}

impl Pane {
	pub fn child(mut self, widget: impl Widget + 'static) -> Self {
		self.add_child(widget);
		self
	}

	pub fn add_child(&mut self, widget: impl Widget + 'static) {
		self.add_dyn_child(Box::new(widget));
	}

	pub fn add_dyn_child(&mut self, widget: Box<dyn Widget>) {
		self.children.push(widget);
	}

	pub fn pop_child(&mut self) -> Option<Box<dyn Widget>> {
		self.children.pop()
	}
}

pub fn pane() -> Pane {
	Pane::default()
}

impl Widget for Pane {
	fn draw(&self, indent: usize) {
		println!("{}<pane>", "  ".repeat(indent));

		for child in &self.children {
			child.draw(indent + 1);
		}

		println!("{}</pane>", "  ".repeat(indent));
	}

	fn update(&mut self, event: Event) -> bool {
		let mut handled = false;

		for child in &mut self.children {
			handled |= child.update(event);

			if handled {
				break;
			}
		}

		handled
	}
}
