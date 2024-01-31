use crate::Object;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
	pub name: String,
	pub home: String,
	pub bot: bool,
	pub id: u64,
}

impl User {
	pub fn from(name: String, home: String, bot: bool, id: u64) -> User {
		User {
			name,
			home,
			bot,
			id,
		}
	}
}

impl Object for User {
	fn get_id(&self) -> u64 {
		self.id
	}

	fn serialize(&self) -> Vec<u8> {
		[
			&(self.name.len() as u16).to_le_bytes(),
			self.name.as_bytes(),
			&(self.home.len() as u16).to_le_bytes(),
			self.home.as_bytes(),
			&[self.bot as u8],
			&self.id.to_le_bytes(),
		]
		.concat()
	}

	fn deserialize(data: &[u8]) -> Self {
		let username_len = u16::from_le_bytes(data[0..2].try_into().unwrap()) as usize;

        let mut start = 2;

		let username = String::from_utf8(data[start..start + username_len].to_vec()).unwrap();

        start += username_len;

		let home_len = u16::from_le_bytes(
			data[start..start + 2].try_into().unwrap()
		) as usize;

        start += 2;

		let home =
			String::from_utf8(data[start..start + home_len].to_vec())
				.unwrap();

        start += home_len;

		let bot = data[start] != 0;

		let id = u64::from_le_bytes(data[start + 1..].try_into().unwrap());

		User::from(username, home, bot, id)
	}
}

#[cfg(test)]
mod tests {
	use crate::Object;

	use super::User;

	#[test]
	fn test_equivalent() {
		let user1 = User::from(
			"tudbut".to_owned(),
			"test.lokichat.xyz".to_owned(),
			false,
			0,
		);
		let user2 = User::from(
			"tudbut".to_owned(),
			"test.lokichat.xyz".to_owned(),
			false,
			0,
		);

		assert_eq!(user1, user2);
	}

	#[test]
	fn test_serialize() {
		let user1 = User::from(
			"tudbut".to_owned(),
			"test.lokichat.xyz".to_owned(),
			false,
			0,
		);
		let serialized = user1.serialize();

		let user2 = User::deserialize(&serialized);

		assert_eq!(user1, user2);
	}
}
