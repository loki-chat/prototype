use {
	crate::{components::WidgetState, ecs::*, prelude::*},
	std::{
		any::{Any, TypeId},
		collections::hash_map::{Entry, HashMap},
	},
};

#[derive(Default)]
pub struct World {
	archetypes: Vec<Box<dyn AnonymousArchetype>>,
	archetype_map: HashMap<TypeId, usize>,
	entities: usize,
}
impl World {
	pub fn new_entity(&mut self) -> usize {
		let result = self.entities;
		self.entities += 1;
		result
	}

	pub fn query<C: Component + 'static>(&self) -> Option<&Archetype<C>> {
		let id = self.archetype_map.get(&TypeId::of::<C>())?;
		let archetype = (*self.archetypes).get(*id)?;
		archetype.as_any().downcast_ref::<Archetype<C>>()
	}
	pub fn query_mut<C: Component + 'static>(&mut self) -> Option<&mut Archetype<C>> {
		let id = self.archetype_map.get(&TypeId::of::<C>())?;
		let archetype = (*self.archetypes).get_mut(*id)?;
		archetype.as_any_mut().downcast_mut::<Archetype<C>>()
	}

	pub fn query_from_entity<C: Component + 'static>(&self, entity: usize) -> Option<&C> {
		match self.query::<C>() {
			None => None,
			Some(archetype) => archetype.get(entity),
		}
	}
	pub fn query_from_entity_mut<C: Component + 'static>(
		&mut self,
		entity: usize,
	) -> Option<&mut C> {
		match self.query_mut::<C>() {
			None => None,
			Some(archetype) => archetype.get_mut(entity),
		}
	}

	pub fn prep_archetype<C: Component + 'static>(&mut self) {
		let type_id = TypeId::of::<C>();
		if let Entry::Vacant(entry) = self.archetype_map.entry(type_id) {
			self.archetypes
				.push(Box::new(Archetype::<C>::new(self.entities)));
			entry.insert(self.archetypes.len() - 1);
		}
	}

	pub fn insert_component<C: Component + 'static>(&mut self, entity: usize, component: C) {
		// Ensure we have an archetype for this component type already
		self.prep_archetype::<C>();
		let archetype = self.query_mut::<C>().unwrap();
		archetype.set(entity, Some(component));
	}

	#[allow(clippy::result_unit_err)]
	pub fn insert_component_unchecked(
		&mut self,
		entity: usize,
		component: Box<dyn Any>,
	) -> Result<(), ()> {
		let type_id = (*component).type_id();
		let id = self.archetype_map.get(&type_id).ok_or(())?;
		let archetype = (*self.archetypes).get_mut(*id).ok_or(())?;
		archetype.set_unchecked(entity, component)
	}

	/// Get the State of a widget, if it exists
	pub fn query_state<S: ToString + 'static>(&self, entity: usize) -> Option<&S> {
		let state_archetype = self.query::<State>().unwrap();
		let state = state_archetype.get(entity)?;
		state.state.as_any().downcast_ref::<S>()
	}
	pub fn query_state_mut<S: ToString + 'static>(&mut self, entity: usize) -> Option<&mut S> {
		let state_archetype = self.query_mut::<State>().unwrap();
		let state = state_archetype.get_mut(entity)?;
		let widget_state = state.state.as_any_mut().downcast_mut::<WidgetState<S>>()?;
		Some(&mut widget_state.0)
	}
}
