use crate::state::{Lazy, RectState};
use crate::widget::Widget;

use self::wrappers::WithBg;

pub mod button;
pub mod pane;
pub mod text;
pub mod wrappers;

pub trait WidgetExt: Widget {
	fn bg(self, bg: Lazy<RectState>) -> WithBg<Self>
	where
		Self: Sized;
}

impl<T: Widget> WidgetExt for T {
	fn bg(self, state: Lazy<RectState>) -> WithBg<Self>
	where
		Self: Sized,
	{
		WithBg {
			widget: self,
			state,
		}
	}
}
