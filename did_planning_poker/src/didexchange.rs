use did_key::{generate, Document, Ed25519KeyPair, KeyMaterial, KeyPair};
use didcomm_rs::{
    crypto::{CryptoAlgorithm, SignatureAlgorithm},
    Message,
};
use serde_json::Value;

fn example_body() -> String {
    r#"{
        "@id": "a46cdd0f-a2ca-4d12-afbf-2e78a6f1f3ef",
        "@type": "https://didcomm.org/didexchange/1.0/request",
        "~thread": { 
            "thid": "a46cdd0f-a2ca-4d12-afbf-2e78a6f1f3ef",
            "pthid": "032fbd19-f6fd-48c5-9197-ba9a47040470" 
        },
        "label": "Bob",
        "goal_code": "aries.rel.build",
        "goal": "To create a relationship",
        "did": "B.did@B:A",
        "did_doc~attach": {}
      }
    "#
    .to_string()
}

pub fn didexchange_body(send_did: String, invitation: &Value, did_doc: &Document) -> Value {
    let inv_id = invitation["invitation"]["@id"].as_str().unwrap();
    let service = invitation["invitation"]["services"].as_array().unwrap()[0].clone();
    let _inv_did = service["recipientKeys"].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .to_string();
    let mut body: Value = serde_json::from_str(&example_body()).unwrap();
    let doc_json = serde_json::to_string_pretty(did_doc).unwrap();

    body["~thread"]["pthid"] = inv_id.into();
    body["did"] = send_did.into();
    body["did_doc~attach"] = serde_json::from_str(&doc_json).unwrap();

    println!("{}", serde_json::to_string_pretty(&body).unwrap());
    body
}

pub fn build_message(
    send_did: String,
    rec_did: String,
    send_key: KeyPair,
    rec_key: KeyPair,
    body: String,
) -> String {
    let sign_key = generate::<Ed25519KeyPair>(None);
    let message = Message::new()
        //.m_type("didcomm/aip2;env=rfc19")
        .from(&send_did)
        .to(&[rec_did.as_str()])
        //.thid(&inv_id)
        .body(body.as_str())
        .as_jwe(&CryptoAlgorithm::XC20P, Some(rec_key.public_key_bytes()))
        .kid(&hex::encode(sign_key.public_key_bytes()));

    println!(
        "message {}",
        serde_json::to_string_pretty(&message).unwrap()
    );

    message
        .seal_signed(
            &send_key.private_key_bytes(),
            Some(vec![Some(rec_key.public_key_bytes())]),
            SignatureAlgorithm::EdDsa,
            &[sign_key.private_key_bytes(), sign_key.public_key_bytes()].concat(),
        )
        .unwrap()
}

#[cfg(feature = "bin")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mediation::create_invitation;
    use did_key::{generate, DIDCore, KeyMaterial, X25519KeyPair, CONFIG_JOSE_PUBLIC};
    use didcomm_rs::Error;
    use reqwest::header::CONTENT_TYPE;

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_didexchange() -> Result<(), Error> {
        let host = "https://mediator.ssi.quest";
        let send_key = generate::<X25519KeyPair>(None);

        let _send_priv_key = send_key.private_key_bytes();
        let did_doc = send_key.get_did_document(CONFIG_JOSE_PUBLIC);
        let send_did = did_doc.id.clone();

        let invitation: Value =
            create_invitation("did-planning-poker".to_string(), host.to_string()).await;
        println!("{}", serde_json::to_string_pretty(&invitation).unwrap());

        let body = didexchange_body(send_did.to_string(), &invitation, &did_doc);

        //println!("send_did {} inv_did {}", send_did, inv_did);

        let inv_did = invitation["invitation"]["services"][0]["recipientKeys"]
            .as_array()
            .unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();

        let rec_pub_key = inv_did.as_str().as_bytes().to_vec();
        println!("{:?} , inv_did {}", rec_pub_key, inv_did);

        let body = serde_json::to_string(&body).unwrap();

        let pub_key = did_key::resolve(&inv_did).unwrap();
        //let pub_key = pub_key.public_key_bytes();

        let ready_to_send = build_message(send_did, inv_did, send_key, pub_key, body);

        let jws_object: Value = serde_json::from_str(&ready_to_send)?;
        assert!(jws_object["protected"].as_str().is_some());
        println!("{}", serde_json::to_string_pretty(&jws_object)?);

        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/didcomm", host))
            .body(ready_to_send)
            .header(CONTENT_TYPE, "application/didcomm-envelope-enc")
            .send()
            .await
            .unwrap();
        let status = res.status();
        println!("{}", res.text().await.unwrap());
        assert_eq!(status, http::StatusCode::OK);

        Ok(())
    }
}
