use crate::key_from_b58;
use did_key::{generate, DIDCore, KeyMaterial, KeyPair, X25519KeyPair};
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::messagepickup::MessagePickupResponseBuilder;
use didcomm_rs::Message;
use js_sys::Promise;
use serde_json::Value;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::future_to_promise;

#[wasm_bindgen]
pub struct Handler {
    key: KeyPair,
    did: String,
    mediator_did: String,
    host: String,
    callbacks: HashMap<String, js_sys::Function>,
}

#[wasm_bindgen]
impl Handler {
    #[wasm_bindgen(constructor)]
    pub fn new(private_key: String, mediator_host: String, mediator_did: String) -> Self {
        let key = key_from_b58(private_key);
        let did = key.get_did_document(Default::default()).id;
        Handler {
            key,
            did,
            host: mediator_host,
            mediator_did,
            callbacks: HashMap::new(),
        }
    }

    pub fn next(&self) -> Promise {
        let client = reqwest::Client::new();
        let request = MessagePickupResponseBuilder::new()
            .batch_size(5)
            .build_batch_pickup()
            .unwrap()
            .from(&self.did);

        let private_key = self.key.private_key_bytes().clone();
        let host = self.host.to_string();
        let key = generate::<X25519KeyPair>(Some(&private_key.clone()));
        let did_doc = self.key.get_did_document(Default::default());
        let did_from = did_doc.id.to_string();
        let mediator_did = self.mediator_did.clone();

        future_to_promise(async move {
            let request = sign_and_encrypt(&request, &did_from, &mediator_did, &key)
                .await
                .unwrap();
            let response = client.post(host).json(&request).send().await.unwrap();

            if !response.status().is_success() {
                return Err(JsValue::from_str("pickup error"));
            }

            let response_json = response.text().await.unwrap();
            let received = Message::receive(&response_json, Some(&private_key), None, None);

            let messages: Vec<Message> = match received {
                Ok(message) => message
                    .get_attachments()
                    .map(|attachment| {
                        let response_json = attachment.data.json.as_ref().unwrap();
                        Message::receive(response_json, Some(&private_key), None, None).unwrap()
                    })
                    .collect(),
                _ => Vec::new(),
            };
            Ok(JsValue::from_serde(&messages).unwrap())
        })
    }

    pub fn handle(&self, messages: &JsValue) {
        let messages: Vec<Message> = messages.into_serde().unwrap();
        let this = JsValue::null();
        for message in &messages {
            match message.get_didcomm_header().m_type.as_str() {
                "https://didcomm.org/routing/2.0/forward" => {
                    for attachment in message.get_attachments() {
                        let forwarded_json = attachment.data.json.as_ref().unwrap();
                        let forwarded = Message::receive(
                            &forwarded_json,
                            Some(&self.key.private_key_bytes()),
                            None,
                            None,
                        )
                        .unwrap();
                        self.handle(&JsValue::from_serde(&vec![forwarded]).unwrap())
                    }
                }
                "https://didcomm.org/trust-ping/2.0/ping-response" => {
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "ping-response",
                        "did": message.get_didcomm_header().from.as_ref().unwrap(),
                        "to": message.get_didcomm_header().to,
                        "id": message.get_didcomm_header().id.to_string(),
                        "thid": message.get_didcomm_header().thid.as_ref().unwrap().to_string()
                    }))
                    .unwrap();
                    match self.callbacks.get("ping-response") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://didcomm.org/trust-ping/2.0/ping" => {
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "ping",
                        "did": message.get_didcomm_header().from.as_ref().unwrap(),
                        "to": message.get_didcomm_header().to,
                        "id": message.get_didcomm_header().id.to_string(),
                    }))
                    .unwrap();
                    match self.callbacks.get("ping") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://github.com/chriamue/did-planning-poker/blob/main/join.md#join" => {
                    let body: Value = serde_json::from_str(&message.get_body().unwrap()).unwrap();
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "join",
                        "from": message.get_didcomm_header().from.as_ref().unwrap(),
                        "to": message.get_didcomm_header().to,
                        "id": body["id"].as_str().unwrap(),
                        "alias": body["alias"].as_str().unwrap(),
                        "icon": body["icon"].as_str().unwrap(),
                        "did": body["did"].as_str().unwrap(),
                    }))
                    .unwrap();
                    match self.callbacks.get("join") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://github.com/chriamue/did-planning-poker/blob/main/game.md#players" => {
                    let body: Value = serde_json::from_str(&message.get_body().unwrap()).unwrap();
                    let players: Vec<crate::game::Player> =
                        serde_json::from_value(body["players"].clone()).unwrap();
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "players",
                        "id": body["id"].as_str().unwrap(),
                        "players": players
                    }))
                    .unwrap();
                    match self.callbacks.get("players") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://github.com/chriamue/did-planning-poker/blob/main/game.md#cards" => {
                    let body: Value = serde_json::from_str(&message.get_body().unwrap()).unwrap();
                    let cards: Vec<String> = serde_json::from_value(body["cards"].clone()).unwrap();
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "cards",
                        "id": body["id"].as_str().unwrap(),
                        "cards": cards
                    }))
                    .unwrap();
                    match self.callbacks.get("cards") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://github.com/chriamue/did-planning-poker/blob/main/game.md#vote" => {
                    let body: Value = serde_json::from_str(&message.get_body().unwrap()).unwrap();
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "cards",
                        "id": body["id"].as_str().unwrap(),
                        "did": body["did"].as_str().unwrap(),
                        "vote": body["vote"].as_str().unwrap(),
                    }))
                    .unwrap();
                    match self.callbacks.get("vote") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                "https://github.com/chriamue/did-planning-poker/blob/main/game.md#reveal" => {
                    let body: Value = serde_json::from_str(&message.get_body().unwrap()).unwrap();
                    let value = JsValue::from_serde(&serde_json::json!({
                        "type": "cards",
                        "id": body["id"].as_str().unwrap(),
                        "reveal": body["reveal"].as_bool().unwrap()
                    }))
                    .unwrap();
                    match self.callbacks.get("reveal") {
                        Some(f) => {
                            f.call1(&this, &value).unwrap();
                        }
                        _ => (),
                    }
                }
                _ => (),
            }
        }
    }

    pub fn on(&mut self, protocol: String, f: js_sys::Function) {
        self.callbacks.insert(protocol, f);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use wasm_bindgen_test::*;

    #[wasm_bindgen_test]
    fn test_on() {
        let key = crate::wasm::generate_private_key();
        let callback = js_sys::Function::default();
        let mut handler = Handler::new(
            key,
            "http://example.com/didcomm".to_string(),
            "did:example".to_string(),
        );
        handler.on("ping".to_string(), callback);
    }
}
