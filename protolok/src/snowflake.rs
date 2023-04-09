use once_cell::sync::Lazy;
use std::{
	sync::{Arc, Mutex},
	time::SystemTime,
};

static COUNTER: Lazy<Arc<Mutex<u16>>> = Lazy::new(|| Arc::new(Mutex::new(0)));

pub struct Snowflake {
	/// creation time in unix millis, only lower 32 bit are accounted for.
	pub time: u64,
	/// special hash format for domain names: [[SnowHash]]
	/// hash collisions don't matter too much as the rest already makes it quite unique, this is just a sort-of-equivalent to discord's worker ids.
	pub name_of_origin: u16,
	/// simply incremented every creation.
	pub internal_counter: u16,
}

impl Snowflake {
	pub fn new(server: &str) -> Self {
		Snowflake {
			time: SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_millis() as u64,
			name_of_origin: SnowHash::from(server),
			internal_counter: {
				let mut ctr = COUNTER.lock().unwrap();
				(*ctr, *ctr = ctr.wrapping_add(1)).0
			},
		}
	}
}

impl From<Snowflake> for u64 {
	fn from(value: Snowflake) -> Self {
		(value.time as u64).wrapping_shl(12 + 11)
			+ (value.name_of_origin as u64).wrapping_shl(11)
			+ value.internal_counter as u64
	}
}

impl From<u64> for Snowflake {
	fn from(value: u64) -> Self {
		Snowflake {
			time: value >> (12 + 11),
			name_of_origin: (value >> 11) as u16,
			internal_counter: value as u16,
		}
	}
}

/// Very simple but reasonably effective hash for domain names. Not perfect, but doesn't have to be.
pub struct SnowHash;

impl SnowHash {
	pub fn from(string: &str) -> u16 {
		let mut val = 0u16;
		let mut last_char = 0;
		for chr in string.chars() {
			let c = chr as u32;
			if chr.is_ascii_digit() {
				val = val.wrapping_add((c.wrapping_sub('0' as u32)) as u16);
			} else {
				val = val.wrapping_shl(1) + if c.wrapping_sub(last_char) > 0 { 1 } else { 0 };
				val = val.wrapping_add((c & 3) as u16);
			}
			last_char = c;
		}
		val & 0b1_11111_11111
	}
}

#[cfg(test)]
mod test {
	use std::time::SystemTime;

	use super::Snowflake;

	#[test]
	fn snowflakes() {
		println!(
			"generated snowflake for tudbut.de: {}",
			u64::from(Snowflake::new("tudbut.de"))
		);
		println!(
			"generated snowflake for tudbut.de: {}",
			u64::from(Snowflake::new("tudbut.de"))
		);
		println!(
			"generated snowflake for lokichat.wtf: {}",
			u64::from(Snowflake::new("lokichat.wtf"))
		);
		println!(
			"generated snowflake for lokichat.wtf: {}",
			u64::from(Snowflake::new("lokichat.wtf"))
		);
		println!(
			"snowflake time in days since last wrap or UNIX: {}",
			Snowflake::from(u64::from(Snowflake::new("tudbut.de"))).time / 1000 / 60 / 60 / 24
		);
		assert_eq!(
			Snowflake::from(u64::from(Snowflake::new("tudbut.de"))).time,
			SystemTime::now()
				.duration_since(SystemTime::UNIX_EPOCH)
				.unwrap()
				.as_millis() as u64
				& 0b1__11111_11111__11111_11111__11111_11111__11111_11111
		);
	}
}
