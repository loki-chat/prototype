use std::{
	collections::HashMap,
	sync::{Mutex, RwLock},
	thread::{self, ThreadId},
	time::{Duration, SystemTime},
};

/// 2023-01-01 00:00:00.000 GMT
const JAN_1_2023: u64 = 1672531200000;

pub struct Count {
	pub count: u64,
	pub time_ms: u64,
}

pub struct Timekeeper {
	last_invoked: SystemTime,
	current_count: u16,
}

impl Timekeeper {
	pub fn new() -> Self {
		Self {
			last_invoked: SystemTime::now(),
			current_count: 0,
		}
	}
}

impl Default for Timekeeper {
	fn default() -> Self {
		Self::new()
	}
}

impl Iterator for Timekeeper {
	type Item = Count;

	/// Never returns None.
	fn next(&mut self) -> Option<Self::Item> {
		let cnt = self.current_count;
		self.current_count += 1;
		let time = SystemTime::now();
		if time
			.duration_since(self.last_invoked)
			.expect("time should not go backwards!!!!!")
			.as_millis() > 0
		{
			self.current_count = 0;
		}
		Some(Count {
			count: cnt as u64,
			time_ms: time
				.duration_since(SystemTime::UNIX_EPOCH + Duration::from_millis(JAN_1_2023))
				.expect("unix epoch should be minimum")
				.as_millis() as u64,
		})
	}
}

pub struct ServerMeta {
	pub server_domain: &'static str,
	pub worker_id: RwLock<HashMap<ThreadId, u16>>,
	pub timekeeper: RwLock<HashMap<ThreadId, Mutex<Timekeeper>>>,
	pub next_tid: Mutex<u16>,
}

impl ServerMeta {
	pub fn new(server_domain: &'static str) -> Self {
		Self {
			server_domain,
			worker_id: RwLock::new(HashMap::new()),
			timekeeper: RwLock::new(HashMap::new()),
			next_tid: Mutex::new(0),
		}
	}

	fn next_tid(&self) -> u16 {
		let mut n = self.next_tid.lock().expect("poisoned mutex");
		let tid: u16 = *n;
		*n += 1;
		tid
	}

	pub fn get_id(&self) -> u16 {
		let key = thread::current().id();
		let tid = self.next_tid();
		if self.worker_id.read().unwrap().get(&key).is_none() {
			self.worker_id.write().unwrap().insert(key, tid);
		}
		*self.worker_id.read().unwrap().get(&key).unwrap() // SAFETY: Checked above
	}

	pub fn count(&self) -> Count {
		let key = thread::current().id();
		if self.timekeeper.read().unwrap().get(&key).is_none() {
			self.timekeeper
				.write()
				.unwrap()
				.insert(key, Mutex::new(Timekeeper::new()));
		}
		self.timekeeper
			.read()
			.unwrap()
			.get(&key)
			.unwrap() // SAFETY: checked above
			.lock()
			.unwrap()
			.next()
			.unwrap() // SAFETY: Never None.
	}
}

const fn bit_mask(amount: u64) -> u64 {
	let mut m = 0;
	let mut i = 0;
	while i < amount {
		m <<= 1;
		m += 1;
		i += 1;
	}
	m
}

pub fn make_id(meta: &ServerMeta) -> u64 {
	let count = meta.count();
	((count.time_ms & bit_mask(42)) << (10 + 12))
		+ ((meta.get_id() as u64 & bit_mask(10)) << 10)
		+ (count.count & bit_mask(12))
}

pub fn id_to_parts(id: u64) -> (u64, u16, u16) {
	let count = id & bit_mask(12);
	let worker = (id >> 12) & bit_mask(10);
	let time = (id >> (12 + 10)) & bit_mask(42);
	(time, worker as u16, count as u16)
}

pub fn to_unix_time(time: u64) -> u128 {
	time as u128 + JAN_1_2023 as u128
}

pub trait Object {
	fn initialize(&mut self, meta: &ServerMeta);
	fn get_id(&self) -> u64;
	fn id_to_parts(&self) -> (u64, u16, u16) {
		id_to_parts(self.get_id())
	}
	fn get_time(&self) -> SystemTime {
		SystemTime::UNIX_EPOCH
			+ Duration::from_millis(to_unix_time(id_to_parts(self.get_id()).0) as u64)
	}
}
