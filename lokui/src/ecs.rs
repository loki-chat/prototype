use std::any::{Any, TypeId};

pub trait Component {}

pub trait ComponentVec {
	fn type_id(&self) -> TypeId;
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	fn push_none(&mut self);
}
impl<T: Component> ComponentVec for Vec<Option<T>>
where
	T: 'static,
{
	fn type_id(&self) -> TypeId {
		TypeId::of::<T>()
	}

	fn as_any(&self) -> &dyn Any {
		self as &dyn Any
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self as &mut dyn Any
	}

	fn push_none(&mut self) {
		self.push(None);
	}
}

#[derive(Default)]
pub struct World {
	world: Vec<Box<dyn ComponentVec>>,
}
impl World {
	pub fn add_entity(&mut self, entity: Box<dyn Bundle>) {}

	fn register_component<ComponentType: Component + 'static>(
		&mut self,
		entity: usize,
		component: ComponentType,
	) {
		for component_vec in self.world.iter_mut() {
			if let Some(vec) = component_vec
				.as_any_mut()
				.downcast_mut::<Vec<Option<ComponentType>>>()
			{
				if vec.len() < entity {
					for _ in 0..entity - vec.len() {
						vec.push(None);
					}
				}
				vec.insert(entity, Some(component));
				return;
			}
		}
	}
}

pub trait Bundle {
	fn for_each_component(self, function: &mut dyn FnMut(TypeId));
}

macro_rules! impl_for_tuple {
    ( $type:ident, $requirement:ident, $($name:ident)+ ) => (
        impl <$($name: 'static,)+> $type for ($($name,)*) where $($name: $requirement,)+ {
            fn for_each_component(self, function: &mut dyn FnMut(TypeId)) {
                $(
                    function(TypeId::of::<$name>());
                )*
            }
        }
    );

    ( $type:ident, $requirement:ident ) => {
        impl_for_tuple! {$type, $requirement, A}
        impl_for_tuple! {$type, $requirement, A B}
        impl_for_tuple! {$type, $requirement, A B C}
        impl_for_tuple! {$type, $requirement, A B C D}
        impl_for_tuple! {$type, $requirement, A B C D E}
        impl_for_tuple! {$type, $requirement, A B C D E F}
        impl_for_tuple! {$type, $requirement, A B C D E F G}
        impl_for_tuple! {$type, $requirement, A B C D E F G H}
        impl_for_tuple! {$type, $requirement, A B C D E F G H I}
        impl_for_tuple! {$type, $requirement, A B C D E F G H I J}
        impl_for_tuple! {$type, $requirement, A B C D E F G H I J K}
        impl_for_tuple! {$type, $requirement, A B C D E F G H I J K L}
    };
}

impl_for_tuple!(Bundle, Component);

pub struct Entity {
	pub id: usize,
	pub components: Box<dyn Bundle>,
}
