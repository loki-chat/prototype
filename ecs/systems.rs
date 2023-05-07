use {
	crate::{components::TextCallback, ecs::World},
	std::{
		cell::{RefCell, RefMut},
		rc::Rc,
	},
};

pub trait System {
	fn execute(&self, world: RefMut<World>);
}
