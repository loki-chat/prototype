use std::collections::HashMap;

use rand::{CryptoRng, RngCore};
use rsa::{Pkcs1v15Encrypt, PublicKey, RsaPrivateKey};

use crate::ids::{make_id, Object, ServerMeta};

pub struct ClientKeypair<R: CryptoRng + RngCore> {
	pub(crate) rsa_pair: RsaPrivateKey,
	pub rng: R,
	id: u64,
}

impl<R: CryptoRng + RngCore> Object for ClientKeypair<R> {
	fn initialize(&mut self, meta: &ServerMeta) {
		self.id = make_id(meta);
	}

	fn get_id(&self) -> u64 {
		self.id
	}
}

trait Decrypt {
	fn decrypt_content(&self, data: Vec<u8>) -> Option<Vec<u8>>;
}

trait Encrypt {
	fn encrypt_content(&mut self, data: Vec<u8>) -> Option<Vec<u8>>;
}

impl<R: CryptoRng + RngCore> Decrypt for ClientKeypair<R> {
	fn decrypt_content(&self, data: Vec<u8>) -> Option<Vec<u8>> {
		self.rsa_pair.decrypt(Pkcs1v15Encrypt, &data).ok()
	}
}

impl<R: CryptoRng + RngCore> Encrypt for ClientKeypair<R> {
	fn encrypt_content(&mut self, data: Vec<u8>) -> Option<Vec<u8>> {
		self.rsa_pair
			.encrypt(&mut self.rng, Pkcs1v15Encrypt, &data)
			.ok()
	}
}

pub struct ClientCrypto<R: CryptoRng + RngCore> {
	pub keystore: HashMap<u64, ClientKeypair<R>>,
	pub channel_keys: HashMap<u64, u64>,
}
