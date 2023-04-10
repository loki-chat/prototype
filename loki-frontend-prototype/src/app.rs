use std::collections::BTreeMap;

use eframe::egui;

use crate::{
	id::{IdGenerator, UserId},
	model::{Guild, User},
	ui::views::main::MainView,
};

pub struct App {
	view: View,
	state: State,
}

impl App {
	pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
		let mut id_gen = IdGenerator::new();

		let current_user_id: UserId = id_gen.generate();

		let mut state = State {
			current_user: current_user_id,
			guilds: vec![],
			users: BTreeMap::new(),
			id_gen,
		};
		state.users.insert(
			current_user_id,
			User {
				id: current_user_id,
				username: "John Doe".to_string(),
			},
		);
		state.users.insert(
			UserId(0),
			User {
				id: UserId(0),
				username: "Invalid User".into(),
			},
		);

		Self {
			view: View::Main(MainView::default()),
			state,
		}
	}
}

impl eframe::App for App {
	fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
		egui::CentralPanel::default().show(ctx, |ui| {
			match &mut self.view {
				View::Main(view) => view.ui(ui, &mut self.state),
			};
		});
	}
}

enum View {
	Main(MainView),
}

pub struct State {
	pub current_user: UserId,
	pub guilds: Vec<Guild>,
	pub users: BTreeMap<UserId, User>,
	pub id_gen: IdGenerator,
}
