use crate::layout::Padding;
use crate::state::{Lazy, RectState};
use crate::widget::Widget;

use self::wrappers::{WithBg, WithPadding};

pub mod button;
pub mod pane;
pub mod text;
pub mod wrappers;

pub trait WidgetExt: Widget {
	fn bg(self, bg: Lazy<RectState>) -> WithBg<Self>
	where
		Self: Sized;

	fn padding(self, padding: Padding) -> WithPadding<Self>
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

	fn padding(self, padding: Padding) -> WithPadding<Self>
	where
		Self: Sized,
	{
		WithPadding {
			widget: self,
			padding,
		}
	}
}
