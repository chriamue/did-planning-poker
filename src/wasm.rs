use wasm_bindgen::prelude::*;
use base58::ToBase58;
use did_key::{generate, KeyMaterial, X25519KeyPair};

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn generate_private_key() -> String {
    let key = generate::<X25519KeyPair>(None);
    key.private_key_bytes().to_base58()
}