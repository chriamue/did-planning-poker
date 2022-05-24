use crate::game::Player;
use crate::key_from_b58;
use base58::ToBase58;
use did_key::{generate, DIDCore, KeyMaterial, X25519KeyPair};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn init_panic_hook() {
    console_error_panic_hook::set_once();
}

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

#[wasm_bindgen]
pub async fn send_ping(
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    crate::ping::send_ping(&key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub async fn send_pong(
    thid: String,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    crate::ping::send_pong(thid, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub async fn send_join(
    session: String,
    alias: String,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    crate::join::send_join(session, alias, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub async fn send_players(
    id: String,
    players: JsValue,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    let players: Vec<Player> = players.into_serde().unwrap();
    crate::game::send_players(id, players, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub async fn send_cards(
    id: String,
    cards: JsValue,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    let cards: Vec<String> = cards.into_serde().unwrap();
    crate::game::send_cards(id, cards, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}
#[wasm_bindgen]
pub async fn send_vote(
    id: String,
    vote: String,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    crate::game::send_vote(id, vote, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub async fn send_reveal(
    id: String,
    reveal: bool,
    private_key: String,
    did_to: String,
    did_mediator: String,
    host: String,
) -> String {
    let key = key_from_b58(private_key);
    crate::game::send_reveal(id, reveal, &key, did_to, did_mediator, host)
        .await
        .unwrap()
}

#[wasm_bindgen]
pub fn did_from_b58(private_key: String) -> String {
    let key = key_from_b58(private_key);
    let did_doc = key.get_did_document(Default::default());
    did_doc.id
}
