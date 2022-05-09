use did_key::*;
use did_key::{generate, DIDCore, Ed25519KeyPair};
use didcomm_rs::{
    crypto::{CryptoAlgorithm, SignatureAlgorithm, Signer},
    Error, JwmHeader, Message, MessageType,
};
use rand_core::OsRng;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use url::Url;
use uuid::Uuid;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mediation::create_invitation;
    use didcomm_rs::Error;
    use reqwest::header::{ACCEPT, CONTENT_TYPE};
    use std::{thread, time};

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_ping() -> Result<(), Error> {
        let sign_keypair = ed25519_dalek::Keypair::generate(&mut OsRng);
        let key = generate::<Ed25519KeyPair>(None);
        let did_doc = key.get_did_document(Config::default());
        let did = did_doc.id.clone();

        let mut invitation: Value = create_invitation("did-planning-poker".to_string()).await;
        println!("{}", invitation);

        let jws_string = Message::new()
            .m_type("https://didcomm.org/trust-ping/2.0/ping")
            .from(&did)
            .body(r#"{"response_requested": true}"#)
            .as_flat_jws(&SignatureAlgorithm::EdDsa)
            .sign(SignatureAlgorithm::EdDsa.signer(), &sign_keypair.to_bytes())?;
        let jws_object: Value = serde_json::from_str(&jws_string)?;
        assert!(jws_object["protected"].as_str().is_some());
        println!("{}", jws_object);

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8081/")
            .header(CONTENT_TYPE, "application/ssi-agent-wire")
            .body(jws_string)
            .send()
            .await
            .unwrap();
        println!("{}", res.text().await.unwrap());

        Ok(())
    }
}
