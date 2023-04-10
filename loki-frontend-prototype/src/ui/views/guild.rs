use eframe::egui::{
	CentralPanel, Frame, Margin, RichText, ScrollArea, SidePanel, TopBottomPanel, Ui,
};

use crate::{
	app::State,
	id::{ChannelId, GuildId, UserId},
	model::{Message, User},
};

pub struct GuildView {
	id: GuildId,
	channel_id: ChannelId,
	message_field: String,
}

impl GuildView {
	pub fn new(id: GuildId, channel_id: ChannelId) -> Self {
		Self {
			id,
			channel_id,
			message_field: String::new(),
		}
	}

	pub fn ui(&mut self, ui: &mut Ui, state: &mut State) {
		let Some(guild) = state.guilds.iter_mut().find(|guild| guild.id == self.id) else {
			ui.heading("Error");
			ui.label(format!("Non-existent guild ID: {}", self.id));
			return;
		};

		SidePanel::left("guild/sidebar").show_inside(ui, |ui| {
			ui.vertical(|ui| {
				ui.label(RichText::new(&guild.name).strong());
				ui.separator();
				for channel in &guild.channels {
					ui.button(format!("#{}", channel.name));
				}
			});
		});

		let Some(channel) = guild.channels.iter_mut().find(|channel| channel.id == self.channel_id) else {
			ui.heading("Error");
			ui.label(format!("Non-existent channel ID: {}", self.channel_id));
			return;
		};

		TopBottomPanel::bottom("guild/channel/bottom").show_inside(ui, |ui| {
			ui.text_edit_multiline(&mut self.message_field);
			if ui.button("Send").clicked() {
				channel.messages.push(Message {
					id: state.id_gen.generate(),
					author: state.current_user,
					contents: self.message_field.clone(),
				});
				self.message_field.clear();
			}
		});

		CentralPanel::default().show_inside(ui, |ui| {
			ScrollArea::vertical().show(ui, |ui| {
				for message in &channel.messages {
					Frame::none()
						.inner_margin(Margin::symmetric(0.0, 4.0))
						.show(ui, |ui| {
							// TODO: More reusable way of displaying an invalid
							// user.
							let invalid_user = User {
								id: UserId::INVALID,
								username: "Invalid User".into(),
							};
							let user = state.users.get(&message.author).unwrap_or(&invalid_user);
							ui.strong(&user.username);
							ui.label(&message.contents);
						});
				}
			});
		});
	}
}
