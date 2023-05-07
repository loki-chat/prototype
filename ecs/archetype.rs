use {
	crate::ecs::*,
	std::any::{Any, TypeId},
};

/// Stores components in a vector
pub struct Archetype<C: Component> {
	components: Vec<Option<C>>,
}
impl<C: Component + 'static> Archetype<C> {
	pub fn new(size: usize) -> Self {
		let mut components = Vec::with_capacity(size);
		for _ in 0..(size) {
			components.push(None);
		}

		Self { components }
	}

	pub fn get_somes(&self) -> Vec<&C> {
		self.components
			.iter()
			.filter_map(|component| component.as_ref())
			.collect()
	}

	fn verify_length(&mut self, test: usize) {
		if self.components.len() < test {
			for _ in 0..(test - self.components.len()) {
				self.components.push(None);
			}
		}
	}

	pub fn get(&self, entity: usize) -> Option<&C> {
		if entity < self.components.len() {
			self.components[entity].as_ref()
		} else {
			None
		}
	}
	pub fn get_mut(&mut self, entity: usize) -> Option<&mut C> {
		self.verify_length(entity);
		self.components[entity].as_mut()
	}

	pub fn set(&mut self, entity: usize, component: Option<C>) {
		self.verify_length(entity);
		self.components.insert(entity, component);
	}

	#[allow(clippy::result_unit_err)]
	pub fn set_unchecked(&mut self, entity: usize, raw_component: Box<dyn Any>) -> Result<(), ()> {
		match raw_component.downcast::<C>() {
			Ok(component) => {
				self.verify_length(entity);
				self.components.insert(entity, Some(*component));
				Ok(())
			}
			Err(_) => Err(()),
		}
	}
}

/// Type-erased Archetypes
pub trait AnonymousArchetype {
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	#[allow(clippy::result_unit_err)]
	fn set_unchecked(&mut self, entity: usize, component: Box<dyn Any>) -> Result<(), ()>;
	fn contained_type_id(&self) -> TypeId;
}

impl<T: Component + 'static> AnonymousArchetype for Archetype<T> {
	fn as_any(&self) -> &dyn Any {
		self as &dyn Any
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self as &mut dyn Any
	}
	fn set_unchecked(&mut self, entity: usize, component: Box<dyn Any>) -> Result<(), ()> {
		self.set_unchecked(entity, component)
	}
	fn contained_type_id(&self) -> TypeId {
		TypeId::of::<T>()
	}
}
