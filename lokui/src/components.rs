use {
	crate::ecs::*,
	lokui_macros::component,
	miniquad::*,
	skia_safe::*,
	std::{any::Any, cell::RefMut, rc::Rc},
};

// region: Drawable

#[component]
#[derive(Default)]
pub struct Drawable {
	pub bg: Color,
}
impl Drawable {
	pub fn draw(&self, canvas: &mut Canvas, pos: &Position) {
		let rect = Rect::from_xywh(pos.x, pos.y, pos.width, pos.height);
		let mut paint = Paint::default();
		paint.set_anti_alias(true);

		paint.set_stroke(false);
		paint.set_color(self.bg);

		canvas.draw_rect(rect, &paint);
	}

	pub fn new() -> Self {
		Self::default()
	}

	pub fn colour(mut self, colour: Color) -> Self {
		self.bg = colour;
		self
	}
}

// endregion: Drawable

// region: Clickable

type ClickableCallbackFn<Q> = dyn Fn(MouseButton, &mut Query<Q>);
pub struct ClickableCallback<Q: QuerySet>(pub Box<ClickableCallbackFn<Q>>);

pub trait ClickableSystem {
	fn execute(&self, btn: MouseButton, world: &mut World);
}
pub trait IntoClickableSystem<Result: ClickableSystem> {
	fn into_system(self) -> Result;
}

impl<F, Q> IntoClickableSystem<ClickableCallback<Q>> for F
where
	Q: QuerySet,
	F: Fn(MouseButton, &mut Query<Q>) + 'static,
{
	fn into_system(self) -> ClickableCallback<Q> {
		ClickableCallback(Box::new(self))
	}
}
impl<Q: QuerySet> ClickableSystem for ClickableCallback<Q> {
	fn execute(&self, btn: MouseButton, world: &mut World) {
		let mut query = Query::new(world);
		self.0(btn, &mut query);
		query.release(world);
	}
}

#[component]
pub struct Clickable {
	pub callback: Rc<dyn ClickableSystem>,
}
impl Clickable {
	pub fn new<S: ClickableSystem + 'static>(
		callback: impl IntoClickableSystem<S> + 'static,
	) -> Self {
		Self {
			callback: Rc::new(callback.into_system()),
			entity: None,
		}
	}
}

// endregion: Clickable

// region: Parent

#[component]
#[derive(Default)]
pub struct Parent {
	children: Vec<usize>,
}
impl Parent {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn add_child(&mut self, child: usize) {
		self.children.push(child);
	}

	pub fn get_children(&self) -> &Vec<usize> {
		&self.children
	}
}

// endregion: Parent

// region: Child

#[component]
pub struct Child {
	parent: usize,
}
impl Child {
	pub fn new(parent: usize) -> Self {
		Self {
			parent,
			entity: None,
		}
	}

	pub fn get_parent(&self) -> usize {
		self.parent
	}
}

// endregion: Child

// region: Text

#[component]
#[derive(Debug)]
pub struct Text {
	pub text: String,
	pub font: Font,
	pub paint: Paint,
}
impl Text {
	pub fn new(text: String) -> Self {
		Self {
			text,
			font: Font::default(),
			paint: Paint::default(),
			entity: None,
		}
	}

	pub fn write(&self, canvas: &mut Canvas, pos: &Position) {
		if let Some(blob) = TextBlob::new(&self.text, &self.font) {
			let height = &blob.bounds().height();
			canvas.draw_text_blob(blob, (pos.x, pos.y + height), &self.paint);
		}
	}

	pub fn with_size(mut self, size: impl Into<f32>) -> Self {
		self.font.set_size(size.into());
		self
	}

	pub fn with_colour(mut self, colour: impl Into<Color>) -> Self {
		self.paint.set_color(colour.into());
		self
	}
}

// endregion: Text

// region: State

pub struct WidgetState<T: 'static + ToString>(pub T);
pub trait AnonymousState {
	fn as_string(&self) -> String;
	fn as_any(&self) -> &dyn Any;
	fn as_any_mut(&mut self) -> &mut dyn Any;
	#[allow(clippy::result_unit_err)]
	fn set_value(&mut self, value: Box<dyn Any>) -> Result<(), ()>;
}
impl<T: ToString> AnonymousState for WidgetState<T> {
	fn as_string(&self) -> String {
		self.0.to_string()
	}
	fn as_any(&self) -> &dyn Any {
		self as &dyn Any
	}
	fn as_any_mut(&mut self) -> &mut dyn Any {
		self as &mut dyn Any
	}
	fn set_value(&mut self, value_raw: Box<dyn Any>) -> Result<(), ()> {
		match value_raw.downcast::<T>() {
			Ok(value) => {
				self.0 = *value;
				Ok(())
			}
			Err(_) => Err(()),
		}
	}
}
#[component]
pub struct State {
	pub state: Box<dyn AnonymousState>,
}
impl State {
	pub fn new<T: 'static + ToString>(initial_value: T) -> Self {
		println!("Storing as {}", std::any::type_name::<T>());
		Self {
			state: Box::new(WidgetState(initial_value)) as Box<dyn AnonymousState>,
			entity: None,
		}
	}
	pub fn get_value<T: 'static + ToString>(&self) -> Option<&T> {
		let state = self.state.as_any().downcast_ref::<WidgetState<T>>();
		match state {
			Some(state) => Some(&state.0),
			None => None,
		}
	}
	pub fn get_value_mut<T: 'static + ToString>(&mut self) -> Option<&mut T> {
		let state = self.state.as_any_mut().downcast_mut::<WidgetState<T>>();
		match state {
			Some(state) => Some(&mut state.0),
			None => None,
		}
	}
}

// endregion: State

// region: Position

#[component]
#[derive(Default)]
pub struct Position {
	pub x: f32,
	pub y: f32,
	pub width: f32,
	pub height: f32,
}
impl Position {
	pub fn new() -> Self {
		Self::default()
	}

	pub fn with_pos(mut self, x: impl Into<f32>, y: impl Into<f32>) -> Self {
		self.x = x.into();
		self.y = y.into();
		self
	}
	pub fn with_size(mut self, width: impl Into<f32>, height: impl Into<f32>) -> Self {
		self.width = width.into();
		self.height = height.into();
		self
	}

	pub fn pos(&mut self, x: impl Into<f32>, y: impl Into<f32>) {
		self.x = x.into();
		self.y = y.into();
	}

	pub fn size(&mut self, width: impl Into<f32>, height: impl Into<f32>) {
		self.width = width.into();
		self.height = height.into();
	}
}

// endregion: Position
