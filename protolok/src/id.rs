use std::{
	sync::Mutex,
	time::{Duration, SystemTime, UNIX_EPOCH},
};

const EPOCH: Duration = Duration::from_millis(1672531200000);

const fn mask(amount: u64) -> u64 {
	(1 << amount) - 1
}

pub struct SnowflakeGenerator {
	last_evoked: u64,
	count: Mutex<u16>,
}

impl SnowflakeGenerator {
	pub fn new() -> SnowflakeGenerator {
		SnowflakeGenerator {
			last_evoked: 0,
			count: Mutex::new(0),
		}
	}

	pub fn generate(&mut self, worker_id: u16) -> u64 {
		let timestamp = SystemTime::now()
			.duration_since(UNIX_EPOCH + EPOCH)
			.expect("Time went backwards")
			.as_millis() as u64
			& mask(42);

		let mut count = self.count.lock().expect("Poisoned mutex");

		if self.last_evoked > timestamp {
			*count = 0;
			self.last_evoked = timestamp;
		}

		let id = timestamp << 22 | (worker_id as u64 & mask(10)) << 12 | (*count as u64 & mask(12));

		*count += 1;

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
	use super::SnowflakeGenerator;

	#[test]
	fn test_snowflake_count() {
		let mut snowflake = SnowflakeGenerator::new();

		assert_ne!(snowflake.generate(0), snowflake.generate(0));
	}

    #[test]
    fn test_snowflake_worker() {
        let mut snowflake = SnowflakeGenerator::new();

        assert_ne!(snowflake.generate(1), snowflake.generate(0)); 
    }
}
