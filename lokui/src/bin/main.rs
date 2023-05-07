use lokui::prelude::*;
use skia_safe::Color;

fn main() {
	let app = App::default();

	let counter = WidgetBuilder::new(app.world.clone())
		.component(Position::new().with_pos(100., 100.).with_size(200., 100.))
		.component(State::new(0))
		.component(Text::new(String::new()))
		.mount(app.root)
		.build();

	let _my_button = WidgetBuilder::new(app.world.clone())
		.component(Drawable::new().colour(Color::from(0xff123456)))
		.component(Clickable::new(move |btn, query: &mut Query<State>| {
			let query = query.get_mut();
            let counter_state = query.get_mut(counter).unwrap().get_value_mut::<i32>().unwrap();
            match btn {
                miniquad::MouseButton::Left => *counter_state += 1,
                miniquad::MouseButton::Right => *counter_state -= 1,
                _ => ()
            }
			println!("Hello, world! The {btn:?} mouse button was pressed! The counter is now at: {counter_state}");
		}))
		.component(Position::new().with_pos(100., 100.).with_size(200., 100.))
		.mount(app.root)
		.build();

	app.run();
}
