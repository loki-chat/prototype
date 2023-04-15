#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Event {
	Clicked,
	HoverStart,
	HoverEnd,
}

pub trait Widget {
	fn draw(&self, indent: usize);
	fn update(&mut self, event: Event) -> bool;
}
