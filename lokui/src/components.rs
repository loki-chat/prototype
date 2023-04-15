use bevy_ecs::prelude::*;

#[derive(Component)]
pub struct Drawable {}

#[derive(Component)]
pub struct Clickable {}

#[derive(Component)]
pub struct Text {}

#[derive(Component)]
pub struct State<T>(T);
