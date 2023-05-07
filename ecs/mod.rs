pub mod archetype;
pub mod component;
pub mod world;

pub use {
	archetype::{AnonymousArchetype, Archetype},
	component::Component,
	world::World,
};

// #[macro_export]
// macro_rules! entity {
// 	($($x:expr),*) => {
// 		vec![$(Box::new($x),)*]
// 	};
// }
// pub use entity;
