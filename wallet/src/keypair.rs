use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;


pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl KeyPair{

    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let address = String::from("todo");

        KeyPair {
            signing_key,
            verifying_key,
            address,
        }
    }
}