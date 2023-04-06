use rsa::{PaddingScheme, Pkcs1v15Encrypt, PublicKey, RsaPrivateKey, RsaPublicKey};

use crate::user::UserHandle;

#[derive(Clone)]
pub struct UnencryptedContent {
	pub hash: String,
	pub text: String,
}
#[derive(Clone)]
pub struct EncryptedContent {
	pub hash: String,
	pub enc_text: Vec<u8>,
}

#[derive(Clone)]
pub enum MessageContent {
	Encrypted(EncryptedContent),
	Unencrypted(UnencryptedContent),
}
impl MessageContent {
	fn is_encrypted(&self) -> bool {
		matches!(self, &MessageContent::Encrypted { .. })
	}

	fn decrypt(&self, key: &RsaPrivateKey) -> Result<MessageContent, rsa::errors::Error> {
		match self {
			MessageContent::Encrypted(EncryptedContent { hash, enc_text }) => {
				key.decrypt(Pkcs1v15Encrypt, &enc_text).map(|x| {
					Self::Unencrypted(UnencryptedContent {
						hash: hash.to_owned(),
						text: String::from_utf8(x).unwrap_or_else(|_| "INVALID UTF8".to_owned()),
					})
				})
			}
			MessageContent::Unencrypted(x) => Ok(Self::Unencrypted(x.clone())),
		}
	}
}

pub struct Message {
	id: Option<String>,
	from: Option<UserHandle>,
	content: MessageContent,
}

pub fn encrypt(
	unencrypted: UnencryptedContent,
	key: &RsaPublicKey,
) -> Result<MessageContent, rsa::errors::Error> {
	Ok(MessageContent::Encrypted(EncryptedContent {
		hash: unencrypted.hash,
		enc_text: key.encrypt(
			&mut rand::thread_rng(),
			Pkcs1v15Encrypt,
			&unencrypted.text.as_bytes(),
		)?,
	}))
}
