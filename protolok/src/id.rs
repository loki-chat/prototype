use std::time::{Duration, SystemTime, UNIX_EPOCH};

const EPOCH: Duration = Duration::from_millis(1672531200000);

const fn mask(amount: u64) -> u64 {
	(1 << amount) - 1
}

pub struct Snowflake {
	last_evoked: u64,
	count: u16,
	worker_id: u16,
}

impl Snowflake {
	pub fn new(worker_id: u16) -> Snowflake {
		Snowflake {
			last_evoked: 0,
			count: 0,
			worker_id,
		}
	}

	pub fn generate(&mut self) -> u64 {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH + EPOCH)
			.expect("Time went backwards")
			.as_millis() as u64
			& mask(42);

		if self.last_evoked > timestamp {
			self.count = 0;
			self.last_evoked = timestamp;
		}

		let id = timestamp << 22
			| (self.worker_id as u64 & mask(10)) << 12
			| (self.count as u64 & mask(12));

		self.count += 1;

		id
	}
}

pub trait Object {
	fn get_id(&self) -> u64;
	fn creation_time(&self) -> SystemTime {
		UNIX_EPOCH + EPOCH + Duration::from_millis(self.get_id() >> 22)
	}

	fn serialize(&self) -> Vec<u8>;
	fn deserialize(data: &[u8]) -> Self;
}

#[cfg(test)]
mod tests {
	use super::Snowflake;

	#[test]
	fn test_snowflake_count() {
		let mut snowflake = Snowflake::new(0);

		assert_ne!(snowflake.generate(), snowflake.generate());
	}
}
