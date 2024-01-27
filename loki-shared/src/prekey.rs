use ed25519_dalek::{Signature, Signer, SigningKey};
use x25519_dalek::{EphemeralSecret, PublicKey, ReusableSecret};

pub struct PublicSignedPrekey {
	pubkey: PublicKey,
	signature: Signature,
}

impl PublicSignedPrekey {
	pub(crate) fn from(pubkey: PublicKey, signing_key: SigningKey) -> PublicSignedPrekey {
		let signature = signing_key.sign(&pubkey.to_bytes());

		PublicSignedPrekey { pubkey, signature }
	}

	pub fn serialize(&self) -> [u8; 96] {
		let mut output = [0u8; 96];
		let (left, right) = output.split_at_mut(64);
		left.copy_from_slice(&self.pubkey.to_bytes());
		right.copy_from_slice(&self.signature.to_bytes());

		output
	}
}
