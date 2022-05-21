use base58::{FromBase58, ToBase58};
use did_key::{generate, KeyMaterial, KeyPair, X25519KeyPair};

pub mod didexchange;
pub mod mediation;
pub mod ping;

#[cfg(target_arch = "wasm32")]
pub mod wasm;

pub fn key_from_b58(private_key: String) -> KeyPair {
    generate::<X25519KeyPair>(Some(&private_key.from_base58().unwrap()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_key_from_b58() {
        let key = generate::<X25519KeyPair>(None);
        let private_key = key.private_key_bytes().to_base58();

        let key2 = key_from_b58(private_key);
        assert_eq!(key.public_key_bytes(), key2.public_key_bytes());
    }
}