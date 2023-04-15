use crate::lazy::Laz;
use crate::widget::{Widget, Event};

pub struct Button {
	text: String,
	on_click: Option<Box<dyn FnMut()>>,
	enabled: Laz<bool>,
	hovered: bool,
}

impl Button {
	pub fn on_click(mut self, on_click: impl FnMut() + 'static) -> Self {
		self.on_click = Some(Box::new(on_click));
		self
	}
}

impl Widget for Button {
	fn draw(&self, indent: usize) {
		println!("{}<button>{}</button>", "  ".repeat(indent), &self.text);
	}

	fn update(&mut self, event: Event) -> bool {
		if self.enabled.get() {
			match event {
				Event::Clicked => {
					if let Some(on_click) = self.on_click.as_mut() {
						(on_click)();
					}
				}
				Event::HoverStart => {
					self.hovered = true;
				}
				Event::HoverEnd => {
					self.hovered = false;
				}
			}

			true
		} else {
			false
		}
	}
}

pub fn button(text: impl Into<String>) -> Button {
	Button {
		text: text.into(),
		on_click: None,
		enabled: Laz::new(true),
		hovered: false,
	}
}
