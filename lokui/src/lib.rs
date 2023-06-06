#![allow(clippy::unusual_byte_groupings)]

pub mod anim;
pub mod components;
pub mod events;
pub mod layout;
pub mod state;
pub mod widget;

pub mod prelude {
	pub use crate::components::WidgetExt;

	pub use crate::components::pane::pane;
	pub use crate::components::text::text;

	pub use crate::layout::anchor::Anchor;
	pub use crate::layout::padding::Padding;
	pub use crate::layout::DimScalar::*;
	pub use crate::layout::{Direction, Flex, Layout};
	pub use crate::state::{lazy, Lazy, RectState};
	pub use crate::widget::{Widget, WidgetContainer};
}

pub fn indentation(n: usize) -> String {
	"  ".repeat(n)
}
