// address derivation — coming soon
use sha3::{Digest, Keccak256};
use hex;

pub fn derive_address(pubkey_byter : &[u8]) -> String{
    let mut hasher = Keccak256::new();
    hasher.update(pubkey_byter);

    let hash = hasher.finalize();
    let last_20 = &hash[12..]; 

    return format!("0x{}",hex::encode(last_20));
}