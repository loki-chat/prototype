use crate::ids::{make_id, Object, ServerMeta};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct UserHandle {
	pub name: String,
	pub home: String,
	pub bot: bool,
	pub id: u64,
}

impl Object for UserHandle {
	fn initialize(&mut self, meta: &ServerMeta) {
		self.id = make_id(meta);
	}

	fn get_id(&self) -> u64 {
		self.id
	}
}

#[cfg(test)]
mod tests {
	use std::sync::OnceLock;

	use crate::ids::{id_to_parts, to_unix_time, Object, ServerMeta};

	use super::UserHandle;

	#[test]
	fn init_test() {
		static META: OnceLock<ServerMeta> = OnceLock::new();
		META.get_or_init(|| ServerMeta::new("test.lokichat.xyz"));
		let mut handle = UserHandle {
			name: "TudbuT".to_owned(),
			home: "test.lokichat.xyz".to_owned(),
			bot: false,
			id: 0,
		};
		handle.initialize(META.get().unwrap());
		dbg!(handle.id);
		dbg!(id_to_parts(handle.id));
		dbg!(to_unix_time(id_to_parts(handle.id).0));
	}
}
