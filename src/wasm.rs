use crate::key_from_b58;
use base58::ToBase58;
use did_key::{generate, KeyMaterial, X25519KeyPair};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn generate_private_key() -> String {
    let key = generate::<X25519KeyPair>(None);
    key.private_key_bytes().to_base58()
}

#[wasm_bindgen]
pub async fn ping(private_key: String, did_to: String, host: String) -> u32 {
    let key = key_from_b58(private_key);
    crate::ping::ping(&key, did_to, host).await.unwrap()
}
