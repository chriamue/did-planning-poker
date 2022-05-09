use wasm_bindgen::prelude::*;

pub mod didexchange;
pub mod mediation;
pub mod ping;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[cfg(test)]
mod tests {
    use base58::FromBase58;
    use didcomm_rs::{
        crypto::{CryptoAlgorithm, SignatureAlgorithm},
        Error, Message,
    };

    use arrayref::array_ref;
    use rand_core::OsRng;
    use serde_json::Value;
    use x25519_dalek::{PublicKey, StaticSecret};

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[test]
    fn generate_did() {
        let _sign_keypair = ed25519_dalek::Keypair::generate(&mut OsRng);
    }

    #[ignore = "just for fun"]
    #[tokio::test]
    async fn send_oob() -> Result<(), Error> {
        let alice_private = "6QN8DfuN9hjgHgPvLXqgzqYE3jRRGRrmJQZkd5tL8paR"
            .from_base58()
            .unwrap();
        let bobs_private = "HBTcN2MrXNRj9xF9oi8QqYyuEPv3JLLjQKuEgW9oxVKP"
            .from_base58()
            .unwrap();
        let alice_secret_key: StaticSecret =
            StaticSecret::from(array_ref!(alice_private, 0, 32).to_owned());
        let bob_secret_key: StaticSecret =
            StaticSecret::from(array_ref!(bobs_private, 0, 32).to_owned());
        let alice_public = PublicKey::from(&alice_secret_key);
        let bob_public = PublicKey::from(&bob_secret_key);
        let sign_keypair = ed25519_dalek::Keypair::generate(&mut OsRng);
        let message = Message::new()
            .from("did:key:z6MkiTBz1ymuepAQ4HEHYSF1H8quG5GLVVQR3djdX3mDooWp")
            .to(&["did:key:z6MkjchhfUsD6mmvni8mCdXHw216Xrm9bQe2mBH1P5RDjVJG"])
            .as_flat_jwe(
                &CryptoAlgorithm::XC20P,
                Some(bob_public.to_bytes().to_vec()),
            )
            .kid(&hex::encode(sign_keypair.public.to_bytes()));
        println!("{:?}", message.clone().as_raw_json().unwrap());

        let jwe_string = message.seal_signed(
            &alice_private,
            Some(vec![Some(bob_public.to_bytes().to_vec())]),
            SignatureAlgorithm::EdDsa,
            &sign_keypair.to_bytes(),
        )?;

        let jwe_object: Value = serde_json::from_str(&jwe_string)?;

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8081")
            .json(&jwe_object)
            .send()
            .await
            .unwrap();
        println!("{:?}", res);

        Ok(())
    }
}
