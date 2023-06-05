use crate::ids::{make_id, LokiMeta, Object};

pub struct UserHandle {
	pub name: String,
	pub home: String,
	pub id: u64,
}

impl Object for UserHandle {
	fn initialize(&mut self, meta: &LokiMeta) {
		self.id = make_id(meta);
	}
}

#[cfg(test)]
mod tests {
	use std::sync::OnceLock;

	use crate::ids::{id_to_parts, to_unix_time, LokiMeta, Object};

	use super::UserHandle;

	#[test]
	fn init_test() {
		static META: OnceLock<LokiMeta> = OnceLock::new();
		META.get_or_init(|| LokiMeta::new("test.lokichat.xyz"));
		let mut handle = UserHandle {
			name: "TudbuT".to_owned(),
			home: "test.lokichat.xyz".to_owned(),
			id: 0,
		};
		handle.initialize(META.get().unwrap());
		dbg!(handle.id);
		dbg!(id_to_parts(handle.id));
		dbg!(to_unix_time(id_to_parts(handle.id).0));
	}
}
