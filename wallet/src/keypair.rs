use k256::ecdsa::{SigningKey, VerifyingKey};
use rand::rngs::OsRng;
use crate::address::derive_address;

pub struct KeyPair {
    signing_key: SigningKey,
    verifying_key: VerifyingKey,
    address: String,
}

impl KeyPair{

    pub fn generate() -> Self {
        let signing_key = SigningKey::random(&mut OsRng);
        let verifying_key = VerifyingKey::from(&signing_key);
        let pubkey_bytes = verifying_key.to_encoded_point(true).as_bytes().to_vec();
        let address = derive_address(&pubkey_bytes);

        KeyPair {
            signing_key,
            verifying_key,
            address,
        }
    }
}