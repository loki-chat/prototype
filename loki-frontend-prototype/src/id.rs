use std::{fmt::Display, time::SystemTime};

/// The
const EPOCH: SystemTime = SystemTime::UNIX_EPOCH;

pub trait Id {
	fn value(&self) -> u64;
}

macro_rules! id {
	($($name:ident,)*) => {
		$(
			#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
			pub struct $name(pub u64);

			impl Id for $name {
				fn value(&self) -> u64 {
					self.0
				}
			}

			impl From<u64> for $name {
				fn from(value: u64) -> Self {
					Self(value)
				}
			}

			impl Display for $name {
				fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
					write!(f, "{}", self.0)
				}
			}
		)*
	};
}

id!(ChannelId, GuildId, MessageId, UserId,);

impl UserId {
	pub const INVALID: UserId = UserId(0);
}

pub struct IdGenerator {
	counter: u16,
}

impl IdGenerator {
	pub fn new() -> Self {
		Self { counter: 0 }
	}

	pub fn generate<T: Id + From<u64>>(&mut self) -> T {
		self.counter = self.counter.overflowing_add(1).0;

		// Using `SystemTime` is not ideal for us because it's not monotonic,
		// meaning that if we use `now()` twice, the second time might return
		// an earlier value than the first time. The chances of that causing
		// problems here are unlikely though, so it's fine.
		let time = (SystemTime::now().duration_since(EPOCH))
			.unwrap()
			.as_millis();

		let value = ((time as u64) << 22) + (self.counter & 0b111111111111) as u64;

		T::from(value)
	}
}
