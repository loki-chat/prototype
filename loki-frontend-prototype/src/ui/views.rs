use eframe::egui::{CentralPanel, SidePanel, Ui};

pub struct MainView {
	view: View,
}

impl MainView {
	pub fn ui(&mut self, ui: &mut Ui) {
		SidePanel::left("main/sidebar").show_inside(ui, |ui| {
			ui.vertical(|ui| {
				if ui.button("Home").clicked() {
					self.view = View::Home;
				}
				ui.separator();
				for i in 1..=3 {
					if ui.button(format!("Guild {i}")).clicked() {
						self.view = View::Guild;
					}
				}
				ui.separator();
				if ui.button("Add guild").clicked() {
					self.view = View::AddGuild;
				}
			})
		});

		CentralPanel::default().show_inside(ui, |ui| match self.view {
			View::Home => {
				ui.label("This might be where you access direct messages, manage friends, that kind of stuff.");
			}
			View::Guild => {
				ui.label("Here you will be able to talk in the guild.");
			}
			View::AddGuild => {
				ui.label("Here you will be able to join new guilds.");
			}
		});
	}
}

impl Default for MainView {
	fn default() -> Self {
		Self { view: View::Home }
	}
}

enum View {
	Home,
	Guild,
	AddGuild,
}
