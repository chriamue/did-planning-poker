use crate::key_from_b58;
use did_key::{DIDCore, KeyMaterial, KeyPair};
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::messagepickup::MessagePickupResponseBuilder;
use didcomm_rs::Message;
use js_sys::Promise;
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
            .batch_size(2)
            .build_batch_pickup()
            .unwrap()
            .from(&self.did);

        let request = sign_and_encrypt(&request, &self.mediator_did, &self.key);
        let private_key = self.key.private_key_bytes();
        let host = self.host.to_string();
        future_to_promise(async move {
            let response = client.post(host).json(&request).send().await.unwrap();

            if !response.status().is_success() {
                return Err(JsValue::from_str("pickup error"));
            }

            let response_json = response.text().await.unwrap();
            let received = Message::receive(&response_json, Some(&private_key), None, None);

            assert!(&received.is_ok());
            let message: Message = received.unwrap();

            for attachment in message.get_attachments() {
                let response_json = attachment.data.json.as_ref().unwrap();
                let received =
                    Message::receive(response_json, Some(&private_key), None, None).unwrap();
                println!("{:?}", received);
            }
            Ok(JsValue::default())
        })
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
