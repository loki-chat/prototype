#![allow(clippy::unusual_byte_groupings)]

pub mod anim;
pub mod components;
pub mod events;
pub mod layout;
pub mod state;
pub mod widget;

pub fn indentation(n: usize) -> String {
	"  ".repeat(n)
}
