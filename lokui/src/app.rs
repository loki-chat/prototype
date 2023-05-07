use std::borrow::BorrowMut;

use {
	crate::{components::ClickableSystem, ecs::*, prelude::*},
	miniquad::{conf::Conf, *},
	skia_safe::{Canvas, Color},
	std::{
		any::Any,
		cell::{Ref, RefCell},
		rc::Rc,
	},
};

// region: App

pub struct App {
	pub world: Rc<RefCell<World>>,
	pub systems: Systems,
	pub root: usize,
	pub width: i32,
	pub height: i32,
}
impl Default for App {
	fn default() -> Self {
		// Default miniquad width & height
		Self::new(800, 600)
	}
}
impl App {
	pub fn new(width: i32, height: i32) -> Self {
		let world = Rc::new(RefCell::new(World::default()));
		let root = WidgetBuilder::new(world.clone())
			.component(Parent::new())
			.component(Position::new().with_size(width as f32, height as f32))
			.build();
		Self {
			world,
			systems: Systems::default(),
			root,
			width,
			height,
		}
	}
	pub fn add_system<S: System + 'static>(&mut self, system: impl IntoSystem<S>) {
		self.systems.push(system);
	}
	pub fn run(self) {
		let conf = Conf {
			high_dpi: true,
			window_height: self.height,
			window_width: self.width,
			..Default::default()
		};
		miniquad::start(conf, || Box::new(self));
	}
}

impl EventHandler for App {
	fn update(&mut self, _skia_ctx: &mut skia::SkiaContext) {
		self.systems.run((*self.world).borrow_mut().borrow_mut());
	}

	fn mouse_button_down_event(
		&mut self,
		_skia_ctx: &mut skia::SkiaContext,
		button: MouseButton,
		x: f32,
		y: f32,
	) {
		let clickable = if let Some(clickables) = self.world.borrow().query::<Clickable>() {
			press_child_of(
				self.root,
				clickables,
				self.world.borrow().query::<Position>().unwrap(),
				self.world.borrow().query::<Parent>().unwrap(),
				x,
				y,
			)
		} else {
			None
		};
		if let Some(clickable) = clickable {
			clickable.execute(button, (*self.world).borrow_mut().borrow_mut());
		}
	}

	fn mouse_button_up_event(
		&mut self,
		_skia_ctx: &mut skia::SkiaContext,
		_button: MouseButton,
		_x: f32,
		_y: f32,
	) {
	}

	fn mouse_motion_event(&mut self, _skia_ctx: &mut skia::SkiaContext, _x: f32, _y: f32) {}

	fn touch_event(
		&mut self,
		_skia_ctx: &mut skia::SkiaContext,
		_phase: TouchPhase,
		_id: u64,
		_x: f32,
		_y: f32,
	) {
	}

	fn resize_event(&mut self, skia_ctx: &mut skia::SkiaContext, width: f32, height: f32) {
		skia_ctx.recreate_surface(width as i32, height as i32);
	}

	fn draw(&mut self, skia_ctx: &mut skia::SkiaContext) {
		// Clear the app
		let canvas = &mut skia_ctx.surface.canvas();
		canvas.clear(Color::from(0x00000000));

		// Starting at the app's root, descend the tree, and draw any descendants with Drawable
		draw_children_of(
			self.root,
			self.world.borrow().query::<Drawable>(),
			self.world.borrow().query::<Text>(),
			self.world.borrow().query::<Parent>().unwrap(), // This can't be none, the root widget is a Parent
			self.world.borrow().query::<Position>().unwrap(), // Also can't be none, every widget has a position
			canvas,
		);

		// Apply changes
		skia_ctx.dctx.flush(None);
	}
}

// endregion: App

// region: Helper functions

// Draw any Drawable children of a widget, recursively if those children have children
fn draw_children_of(
	widget_id: usize,
	drawables: Option<&Archetype<Drawable>>,
	text: Option<&Archetype<Text>>,
	parents: &Archetype<Parent>,
	positions: &Archetype<Position>,
	canvas: &mut Canvas,
) {
	// If the widget has a Text component, draw the text
	if let Some(text_widgets) = text {
		if let Some(child) = text_widgets.get(widget_id) {
			if let Some(pos) = positions.get(widget_id) {
				child.write(canvas, pos);
			}
		}
	}
	// If the widget has a Drawable component, draw it
	if let Some(drawable_widgets) = drawables {
		if let Some(child) = drawable_widgets.get(widget_id) {
			if let Some(pos) = positions.get(widget_id) {
				child.draw(canvas, pos);
			}
		}
	}
	// If the widget has children, recurse down those children as well
	if let Some(widget) = parents.get(widget_id) {
		for child_id in widget.get_children() {
			draw_children_of(*child_id, drawables, text, parents, positions, canvas);
		}
	}
}

// Recurse down the widget tree, and try to find the widget that was pressed
fn press_child_of(
	widget_id: usize,
	clickables: &Archetype<Clickable>,
	positions: &Archetype<Position>,
	parents: &Archetype<Parent>,
	x: f32,
	y: f32,
) -> Option<Rc<dyn ClickableSystem>> {
	if let Some(widget) = parents.get(widget_id) {
		for child_id in widget.get_children() {
			if let Some(child) = positions.get(*child_id) {
				if x > child.x
					&& y > child.y && x < (child.x + child.width)
					&& y < (child.y + child.height)
				{
					if parents.get(*child_id).is_some() {
						let child = press_child_of(*child_id, clickables, positions, parents, x, y);
						if child.is_some() {
							return child;
						}
					}
					if let Some(child) = clickables.get(*child_id) {
						return Some(child.callback.clone());
					}
				}
			}
		}
	}
	None
}

// endregion: Helper functions

// region: WidgetBuilder

pub struct WidgetBuilder {
	id: usize,
	components: Vec<Box<dyn Any>>,
	parent: Option<usize>,
	world: Rc<RefCell<World>>,
}
impl WidgetBuilder {
	pub fn new(world: Rc<RefCell<World>>) -> Self {
		let id = (*world).borrow_mut().new_entity();
		Self {
			id,
			components: Vec::new(),
			parent: None,
			world,
		}
		.component(Position::new())
	}

	pub fn component<C: Component + 'static>(mut self, component: C) -> Self {
		(*self.world).borrow_mut().prep_archetype::<C>();
		self.components.push(Box::new(component));
		self
	}

	pub fn mount(mut self, parent_id: usize) -> Self {
		self.parent = Some(parent_id);

		self
	}

	pub fn build(self) -> usize {
		for component in self.components {
			(*self.world)
				.borrow_mut()
				.insert_component_unchecked(self.id, component)
				.unwrap();
		}
		if let Some(parent_id) = self.parent {
			let mut world = (*self.world).borrow_mut();
			if let Some(parent) = world.query_from_entity_mut::<Parent>(parent_id) {
				parent.add_child(self.id);
				world.insert_component(self.id, Child::new(parent_id));
			} else {
				panic!(
					"Widget {} attempted to mount to Widget {}, but it didn't have `Parent`!",
					self.id, parent_id
				);
			}
		}
		self.id
	}
}

// endregion: WidgetBuilder
