use crate::{Object, User};

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Message {
	data: Vec<u8>,
	author: User,
	id: u64,
}

impl Object for Message {
	fn get_id(&self) -> u64 {
		self.id
	}

	fn serialize(&self) -> Vec<u8> {
		[(self.data.len() as u32).to_le_bytes().to_vec(), self.data.clone(), self.id.to_le_bytes().to_vec(), self.author.serialize()].concat()
	}

	fn deserialize(data: &[u8]) -> Self {
		let message_len = u32::from_le_bytes(data[0..4].try_into().unwrap()) as usize;
		let message = data[4..message_len + 4].to_vec();

		let id = u64::from_le_bytes(data[message_len + 4..message_len + 4 + 8].try_into().unwrap());

		let author = User::deserialize(&data[message_len + 4 + 8..]);

        Message {
            data: message,
            author,
            id,
        }
	}
}

#[cfg(test)]
mod tests {
    use crate::Object;
    use crate::User;

    use super::Message;

    #[test]
    fn test_serialize() {
        let user1 = User::from(
			"tudbut".to_owned(),
			"test.lokichat.xyz".to_owned(),
			false,
			0,
		);

        let message = Message {
            data: Vec::new(),
            author: user1,
            id: 0,
        };

        let serialized = message.serialize();
        let deserialized = Message::deserialize(&serialized);

        assert_eq!(deserialized, message)
    }
}
