use std::{
	any::{Any, TypeId},
	collections::hash_map::{Entry, HashMap},
};

// region: Archetype

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
	fn as_any_owned(self: Box<Self>) -> Box<dyn Any>;
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
	fn as_any_owned(self: Box<Self>) -> Box<dyn Any> {
		self as Box<dyn Any>
	}
	fn set_unchecked(&mut self, entity: usize, component: Box<dyn Any>) -> Result<(), ()> {
		self.set_unchecked(entity, component)
	}
	fn contained_type_id(&self) -> TypeId {
		TypeId::of::<T>()
	}
}
// endregion: Archetype

// region: Component

/// Trait for all components that make up UI items
pub trait Component {
	/// Lets the component store the entity it belongs to.
	/// Called automatically by the [`Widget.register_component()`]
	///
	/// [`Widget.register_component()`](Widget::register_component)
	fn set_entity(&mut self, entity: usize);

	/// Convert to std::Any
	fn to_any(self) -> Box<dyn Any>;
}

// endregion: Component

// region: Query

/// A trait for anything that queries data from the world.
pub trait WorldQuery: Component + Sized {
	fn query(world: &mut World) -> Archetype<Self>;
}
impl<C> WorldQuery for C
where
	C: Component + 'static,
{
	fn query(world: &mut World) -> Archetype<Self> {
		// TODO: Handle mismatched types that would result in None
		world.take::<C>().unwrap()
	}
}

// region: QuerySet
/// A trait for a set of queries
pub trait QuerySet {
	type Output: Release;
	fn query_all(world: &mut World) -> Self::Output;
}
impl QuerySet for () {
	type Output = ();
	fn query_all(_world: &mut World) -> Self::Output {}
}
impl<A: WorldQuery + 'static> QuerySet for A {
	type Output = Archetype<A>;
	fn query_all(world: &mut World) -> Self::Output {
		A::query(world)
	}
}
impl<A: WorldQuery + 'static> QuerySet for (A,) {
	type Output = (Archetype<A>,);
	fn query_all(world: &mut World) -> Self::Output {
		(A::query(world),)
	}
}
// endregion: QuerySet

// region: Release
/// Restores data back to the world
pub trait Release {
	fn release(self, world: &mut World);
}
impl Release for () {
	fn release(self, _: &mut World) {}
}
impl<A: WorldQuery + 'static> Release for Archetype<A> {
	fn release(self, world: &mut World) {
		let index = *world.archetype_map.get(&TypeId::of::<A>()).unwrap();
		world.archetypes[index] = Some(Box::new(self) as Box<dyn AnonymousArchetype>);
	}
}
impl<A: WorldQuery + 'static> Release for (Archetype<A>,) {
	fn release(self, world: &mut World) {
		let index = *world.archetype_map.get(&TypeId::of::<A>()).unwrap();
		world.archetypes[index] = Some(Box::new(self.0) as Box<dyn AnonymousArchetype>);
	}
}
// endregion: Release

/// A generic allowing systems to query data from the world
pub struct Query<Q: QuerySet>(Q::Output);
impl<Q: QuerySet> Query<Q> {
	pub fn new(world: &mut World) -> Self {
		Self(Q::query_all(world))
	}
	pub fn get(&self) -> &Q::Output {
		&self.0
	}
	pub fn get_mut(&mut self) -> &mut Q::Output {
		&mut self.0
	}
	pub fn release(self, world: &mut World) {
		self.0.release(world);
	}
}

// endregion: Query

// region: Systems

pub trait System {
	fn execute(&self, world: &mut World);
}
pub trait IntoSystem<Result: System> {
	fn into_system(self) -> Result;
}

type QuerySystemFn<Q> = dyn Fn(&mut Query<Q>);
pub struct QuerySystem<Q: QuerySet>(pub Box<QuerySystemFn<Q>>);

impl<Q: QuerySet> System for QuerySystem<Q> {
	fn execute(&self, world: &mut World) {
		let mut query = Query::new(world);
		self.0(&mut query);
		query.release(world);
	}
}

impl<F, Q> IntoSystem<QuerySystem<Q>> for F
where
	Q: QuerySet,
	F: Fn(&mut Query<Q>) + 'static,
{
	fn into_system(self) -> QuerySystem<Q> {
		QuerySystem(Box::new(self))
	}
}

#[derive(Default)]
pub struct Systems(Vec<Box<dyn System>>);
impl Systems {
	pub fn run(&self, world: &mut World) {
		for system in &self.0 {
			system.execute(world);
		}
	}
	pub fn push<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
		self.0.push(Box::new(system.into_system()));
	}
}

// endregion: Systems

// region: World

#[derive(Default)]
pub struct World {
	archetypes: Vec<Option<Box<dyn AnonymousArchetype>>>,
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
		// If the option has a None, then a value was taken and not returned. This should halt the program instead of
		//  continuing to run with stolen memory, because the library is clearly broken
		let archetype = (*self.archetypes).get(*id)?.as_ref().unwrap();
		archetype.as_any().downcast_ref::<Archetype<C>>()
	}
	pub fn query_mut<C: Component + 'static>(&mut self) -> Option<&mut Archetype<C>> {
		let id = self.archetype_map.get(&TypeId::of::<C>())?;
		let archetype = (*self.archetypes).get_mut(*id)?.as_mut().unwrap();
		archetype.as_any_mut().downcast_mut::<Archetype<C>>()
	}
	pub fn take<C: Component + 'static>(&mut self) -> Option<Archetype<C>> {
		let id = self.archetype_map.get(&TypeId::of::<C>())?;
		let borrowed_archetype = (*self.archetypes).get_mut(*id)?;
		let mut archetype = None;
		std::mem::swap(borrowed_archetype, &mut archetype);
		let archetype = archetype.unwrap();
		Some(*archetype.as_any_owned().downcast::<Archetype<C>>().unwrap())
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
				.push(Some(Box::new(Archetype::<C>::new(self.entities))));
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
		let archetype = (*self.archetypes).get_mut(*id).ok_or(())?.as_mut().unwrap();
		archetype.set_unchecked(entity, component)
	}
}

// endregion: World
