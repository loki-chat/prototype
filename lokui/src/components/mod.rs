use crate::state::Lazy;
use crate::widget::Widget;

use self::wrappers::{BackgroundState, WithBackground};

pub mod button;
pub mod pane;
pub mod text;
pub mod wrappers;

pub trait WidgetExt: Widget {
	fn bg(self, state: Lazy<BackgroundState>) -> WithBackground<Self>
	where
		Self: Sized;
}

impl<T: Widget> WidgetExt for T {
	fn bg(self, state: Lazy<BackgroundState>) -> WithBackground<Self>
	where
		Self: Sized,
	{
		WithBackground {
			widget: self,
			state,
		}
	}
}
