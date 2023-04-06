use std::time::SystemTime;

use sha2::{Digest, Sha256};

pub struct UserHandle {
	name: String,
	home: String,
}

impl UserHandle {
	pub fn get_id(&self) -> String {
		let mut buf = [0u8; 64];
		format!(
			"{:X}",
			SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_millis()
		) + &base16ct::upper::encode_str(&Sha256::digest(self.name.as_bytes()), &mut buf).unwrap()
			[..6] + &base16ct::upper::encode_str(&Sha256::digest(self.home.as_bytes()), &mut buf)
			.unwrap()[..6]
	}
}
