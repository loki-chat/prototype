use crate::id::{ChannelId, GuildId, IdGenerator, MessageId, UserId};

pub struct Guild {
	pub id: GuildId,
	pub name: String,
	pub channels: Vec<Channel>,
}

impl Guild {
	pub fn new(id_gen: &mut IdGenerator, name: String) -> Self {
		Self {
			id: id_gen.generate(),
			name,
			channels: vec![Channel {
				id: id_gen.generate(),
				name: "general".into(),
				messages: vec![],
			}],
		}
	}
}

pub struct Channel {
	pub id: ChannelId,
	pub name: String,
	pub messages: Vec<Message>,
}

pub struct Message {
	pub id: MessageId,
	pub author: UserId,
	pub contents: String,
}

pub struct User {
	pub id: UserId,
	pub username: String,
}
