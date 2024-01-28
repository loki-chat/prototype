use std::collections::hash_map::DefaultHasher;
use std::hash::Hasher;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

const EPOCH: u64 = 1672531200000;

fn generate_id(name: String, home: String, bot: bool) -> u64 {
	let mut hash = DefaultHasher::new();
	hash.write(name.as_bytes());
	hash.write(home.as_bytes());

	let mut id = hash.finish();

	let mut time: u64 = SystemTime::now()
		.duration_since(UNIX_EPOCH + Duration::from_millis(EPOCH))
		.expect("Time went backwards")
		.as_millis() as u64;

    time |= bot as u64;

	id >>= 32;
	id <<= 32;
	id |= time;

	id
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct User {
	pub name: String,
	pub home: String,
	pub bot: bool,
	pub id: u64,
}

impl User {
	pub fn new(name: String, home: String, bot: bool) -> User {
		let id = generate_id(name, home, bot);
		User {
			name,
			home,
			bot,
			id,
		}
	}
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_unique() {
        let user1 = User::new("tudbut", "loki.chat", false);
        let user2 = User::new("tudbut", "loki.chat", true);
        
        assert_ne!(user1, user2);
    }
}
