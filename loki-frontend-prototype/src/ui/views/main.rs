use std::default::Default;

use eframe::egui::{CentralPanel, RichText, SidePanel, Ui};

use crate::{app::State, id::GuildId, model::Guild};

use super::guild::GuildView;

pub struct MainView {
	view: View,
}

impl MainView {
	pub fn ui(&mut self, ui: &mut Ui, state: &mut State) {
		SidePanel::left("main/sidebar").show_inside(ui, |ui| {
			ui.vertical(|ui| {
				if ui.button("Home").clicked() {
					self.view = View::Home;
				}
				ui.separator();
				// `.iter()` avoids unnecessary mutable borrow.
				for guild in state.guilds.iter() {
					if ui.button(&guild.name).clicked() {
						self.view = View::Guild(GuildView::new(guild.id, guild.channels[0].id));
					}
				}
				ui.separator();
				if ui.button("Add guild").clicked() {
					self.view = View::AddGuild(AddGuildView::default());
				}
			})
		});

		CentralPanel::default().show_inside(ui, |ui| match &mut self.view {
			View::Home => {
				ui.label("This might be where you access direct messages, manage friends, that kind of stuff.");
			}
			View::Guild(view) => view.ui(ui, state),
			View::AddGuild(view) => view.ui(ui, state),
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
	Guild(GuildView),
	AddGuild(AddGuildView),
}

#[derive(Default)]
struct AddGuildView {
	guild_name: String,
}

impl AddGuildView {
	pub fn ui(&mut self, ui: &mut Ui, state: &mut State) {
		ui.heading("Create guild");
		ui.text_edit_singleline(&mut self.guild_name);
		if ui.button("Create").clicked() {
			state
				.guilds
				.push(Guild::new(&mut state.id_gen, self.guild_name.clone()));
		}
	}
}
