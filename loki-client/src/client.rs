use protolok::user::UserHandle;
use rsa::{RsaPrivateKey, RsaPublicKey};

pub struct KeyPair(RsaPublicKey, RsaPrivateKey);

pub struct Client {
	client_user: UserHandle,
	keypair: KeyPair,
}
