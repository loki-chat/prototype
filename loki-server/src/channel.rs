use std::{mem::take, sync::RwLock};

use protolok::{message::Message, Channel, Object, ServerMeta, UserHandle};
use rsa::{rand_core::OsRng, RsaPrivateKey, RsaPublicKey};

use crate::errors::InUseError;

pub struct MessageStore {
	messages: Vec<Message>,
	/// encryption pubkey, if channel has any encrypted messages. used for checking messages
	/// and for identifying fragments
	pubkey: RsaPublicKey,
	/// when a member leaves, a rebuild must be done to ensure privacy of past messages
	needs_rebuild: bool,
}
impl MessageStore {
	pub fn new(pubkey: RsaPublicKey, needs_rebuild: bool) -> Self {
		Self {
			messages: Vec::new(),
			pubkey,
			needs_rebuild,
		}
	}
}

pub struct ServerChannel {
	participants: Vec<UserHandle>,
	message_store: RwLock<Vec<RwLock<MessageStore>>>,
}

impl ServerChannel {
	pub fn new() -> Self {
		Self {
			participants: Vec::new(),
			message_store: RwLock::new(Vec::new()),
		}
	}

	pub fn defragment(&self) -> Result<(), InUseError> {
		let r_messages = self.message_store.read().unwrap();
		if r_messages.len() > 1 {
			drop(r_messages);
			let mut w_messages = self
				.message_store
				.try_write()
				.map_err(|_| InUseError("Unable to lock channel for defragmentation."))?;
			let messages = take(&mut *w_messages);
			unify(messages, &mut w_messages)
		}
		Ok(())
	}
}

impl Default for ServerChannel {
	fn default() -> Self {
		Self::new()
	}
}

fn unify(stores: Vec<RwLock<MessageStore>>, field: &mut Vec<RwLock<MessageStore>>) {
	for message_store in stores {
		let mut message_store = message_store.into_inner().unwrap();
		if let Some(store) = field.last() {
			let mut store = store.write().unwrap();
			if store.pubkey == message_store.pubkey {
				store.messages.append(&mut message_store.messages);
			}
		}
		if !message_store.messages.is_empty() {
			field.push(RwLock::new(message_store));
		}
	}
}

#[derive(Debug)]
pub enum NeverError {}

impl Channel<NeverError, &ServerMeta> for ServerChannel {
	/// on the server side, no limit is set to fetching, but it should be ensured not too much is fetched to avoid slow-downs.
	/// FETCH WILL DELETE AN EMPTY BLOCK WHEN ENCOUNTERED => THIS IS NOT READONLY AND MUST NOT BE USED WHILE WRITE-LOCKED.
	fn fetch_newest_messages(
		&self,
		mut amount: u32,
		begin_at_id: Option<u64>,
	) -> Result<Vec<Message>, NeverError> {
		let begin_at_id = begin_at_id.unwrap_or(u64::MAX);
		'i: loop {
			let message_store = self.message_store.read().unwrap();
			let mut msgs = Vec::new();
			if amount == 0 {
				break 'i Ok(msgs);
			}

			///////////////////////////////////////
			// binary search for the start block //
			///////////////////////////////////////

			let mut min_idx = 0;
			let mut max_idx = message_store.len();
			let mut idx = 0;
			let mut last_idx = 1;
			let start = loop {
				if last_idx == idx {
					break &message_store[idx];
				}
				let mut further = true;
				let store = message_store[idx].read().unwrap();
				if let Some(message) = store.messages.last() {
					if message.id <= begin_at_id {
						min_idx = idx;
					} else {
						max_idx = idx;
						further = false;
					}
				} else {
					drop(store);
					drop(message_store);
					self.message_store.write().unwrap().remove(idx);
					// restart search
					continue 'i;
				}
				last_idx = idx;
				idx = (min_idx + max_idx) / 2
					+ if further {
						max_idx - min_idx
					} else {
						min_idx + max_idx
					} / 4;
			};
			let start = start.read().unwrap();
			println!("Found start. IDX={idx}");

			///////////////////////////
			// actually get messages //
			///////////////////////////

			// binary search for the start message
			let mut start_msg_idx = start
				.messages
				.binary_search_by(|x| x.id.cmp(&begin_at_id).reverse())
				.unwrap_or_else(|e| e);
			// then go through all messages from there, adding them to msgs in the process
			// 1. go through blocks starting at start block
			for idx in idx..message_store.len() {
				let store = message_store[idx].read().unwrap();
				// 2. go through messages of the block.
				// on the start block, start_msg_idx will still be set, and will cause it to start from there
				for i in start_msg_idx..store.messages.len() {
					if amount == 0 {
						break 'i Ok(msgs);
					}
					amount -= 1;
					msgs.push(store.messages[i].clone());
				}
				// reset start_msg_idx so that blocks after the start block actually start at the beginning
				start_msg_idx = 0;
			}
			break 'i Ok(msgs);
		}
	}

	fn send_message(
		&self,
		message: protolok::MessageContent,
		ctx: &ServerMeta,
	) -> Result<Message, NeverError> {
		let mut message_store = self.message_store.write().unwrap();
		if message_store.is_empty() {
			message_store.push(RwLock::new(MessageStore::new(
				RsaPrivateKey::new(&mut OsRng, 256)
					.expect("unable to generate temporary key!")
					.to_public_key(),
				true,
			)));
		}
		let mut message = Message {
			id: 0,
			from: Some(UserHandle {
				name: "Loki".to_owned(),
				home: ctx.server_domain.to_owned(),
				bot: true,
				id: 0,
			}),
			content: message,
		};
		message.initialize(ctx);
		message_store
			.last()
			.unwrap()
			.write()
			.unwrap()
			.messages
			.push(message.clone());
		Ok(message)
	}
}

#[cfg(test)]
mod tests {
	use std::{
		collections::HashMap,
		sync::{Mutex, RwLock},
	};

	use super::ServerChannel;
	use protolok::{Channel, MessageContent, ServerMeta, UnencryptedContent};

	#[test]
	pub fn send_and_receive() {
		let meta = ServerMeta {
			server_domain: "test.localhost",
			worker_id: RwLock::new(HashMap::new()),
			timekeeper: RwLock::new(HashMap::new()),
			next_tid: Mutex::new(0),
		};
		let ctx = &meta;
		let channel = ServerChannel::new();
		let content1 = MessageContent::Unencrypted(UnencryptedContent {
			hash: "ignore".to_owned(),
			text: "TESTMSG1".to_owned(),
		});
		let content2 = MessageContent::Unencrypted(UnencryptedContent {
			hash: "ignore".to_owned(),
			text: "TESTMSG2".to_owned(),
		});
		let msg1 = channel.send_message(content1.clone(), ctx).unwrap();
		let msg2 = channel.send_message(content2.clone(), ctx).unwrap();
		let msgs = channel.fetch_newest_messages(5, None).unwrap();
		assert_eq!(msgs.len(), 2);
		assert_eq!(msgs[0], msg1);
		assert_eq!(msgs[1], msg2);
		assert_eq!(msgs[0].content, content1);
		assert_eq!(msgs[1].content, content2);
	}
}
