use chacha20poly1305::{
	aead::{rand_core::RngCore, Aead, KeyInit, OsRng},
	Key, XChaCha20Poly1305, XNonce,
};
use std::error::Error;
use std::fmt;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone, Copy, PartialEq)]
pub struct InvalidKey;

impl Eq for InvalidKey {}

impl fmt::Display for InvalidKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Invalid key")
	}
}

impl fmt::Debug for InvalidKey {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "Invalid key")
	}
}

impl Error for InvalidKey {}

type Cipher = XChaCha20Poly1305;

pub struct Message {
	data: Vec<u8>,
	nonce: Option<Vec<u8>>,
	encrypted: bool,
}

impl Message {
	pub fn encrypt(key: &[u8], data: String, raw_nonce: Option<&[u8]>) -> Message {
		let nonce = if raw_nonce.is_none() {
			let timestamp = SystemTime::now()
				.duration_since(UNIX_EPOCH)
				.expect("Time went backwards")
				.as_nanos()
				.to_le_bytes();

			let mut random = [0u8; 8];
			OsRng.fill_bytes(&mut random);
			let x: Vec<u8> = [timestamp.as_ref(), random.as_ref()].concat();
			XNonce::from_iter(x)
		} else {
			XNonce::from_iter(raw_nonce.unwrap().to_vec())
		};

		let cipher = Cipher::new(Key::from_slice(key));

		Message {
			data: cipher.encrypt(&nonce, data.as_bytes()).unwrap(), // *hopefully* shouldn't panic
			nonce: Some(nonce.to_vec()),
			encrypted: true,
		}
	}

	pub fn new(data: String) -> Message {
		Message {
			data: data.as_bytes().to_vec(),
			nonce: None,
			encrypted: false,
		}
	}

	pub fn deserialize(msg: Vec<u8>) -> Message {
		if msg.last().unwrap() == &0u8 {
			Message {
				data: msg[..msg.len() - 2].to_vec(),
				nonce: None,
				encrypted: false,
			}
		} else {
			Message {
				data: msg[24..msg.len() - 2].to_vec(),
				nonce: Some(msg[0..24].to_vec()),
				encrypted: true,
			}
		}
	}

	pub fn serialize(&self) -> Vec<u8> {
		if !self.encrypted {
			[self.data.as_ref(), [0].as_ref()].concat().to_vec()
		} else {
			[
				self.nonce.clone().unwrap().as_ref(),
				self.data.as_ref(),
				[1].as_ref(),
			]
			.concat()
			.to_vec()
		}
	}

	pub fn retrieve(&self, key: &[u8]) -> Result<Vec<u8>, InvalidKey> {
		if !self.encrypted {
			Ok(self.data.clone())
		} else {
			match Cipher::new(Key::from_slice(key)).decrypt(
				XNonce::from_slice(&self.nonce.clone().unwrap()),
				self.data.as_ref(),
			) {
				Ok(x) => Ok(x),
				Err(_) => Err(InvalidKey),
			}
		}
	}
}
