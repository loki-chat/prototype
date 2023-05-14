use crate::events::Event;
use crate::state::{Lazy, RectState};
use crate::widget::Widget;

use self::wrappers::*;

pub mod pane;
pub mod text;
pub mod wrappers;

pub trait WidgetExt: Widget {
	fn bg(self, bg: Lazy<RectState>) -> WithBg<Self>
	where
		Self: Sized;

	fn on_click(self, callback: impl FnMut(f32, f32) + 'static) -> WithOnClick<Self>
	where
		Self: Sized;

	fn on_event(self, callback: impl FnMut(Event) -> bool + 'static) -> WithOnEvent<Self>
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

	fn on_click(self, callback: impl FnMut(f32, f32) + 'static) -> WithOnClick<Self>
	where
		Self: Sized,
	{
		WithOnClick {
			widget: self,
			on_click: Box::new(callback),
			is_mouse_down: false,
		}
	}

	fn on_event(self, callback: impl FnMut(Event) -> bool + 'static) -> WithOnEvent<Self>
	where
		Self: Sized,
	{
		WithOnEvent {
			widget: self,
			callback: Box::new(callback),
		}
	}
}
