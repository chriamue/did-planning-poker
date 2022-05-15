use did_key::*;
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::trustping::TrustPingResponseBuilder;
use instant::Instant;

pub async fn ping(key: &KeyPair, did_to: String, host: String) -> Result<u32, &'static str> {
    let did_doc = key.get_did_document(Default::default());
    let did_from = did_doc.id.to_string();

    let request = TrustPingResponseBuilder::new()
        .build()
        .unwrap()
        .from(&did_from);

    let request = sign_and_encrypt(&request, &did_to, &key);

    let start =  Instant::now();
    let client = reqwest::Client::new();
    let res = client.post(host).json(&request).send().await.unwrap();

    if !res.status().is_success() {
        return Err("ping failed");
    }

    let duration = start.elapsed();

    Ok(duration.as_millis() as u32)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mediation::create_invitation;
    use didcomm_rs::Error;
    use serde_json::Value;

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_ping() -> Result<(), Error> {
        let key = generate::<X25519KeyPair>(None);
        let host = "http://localhost:8000/didcomm".to_string();

        let invitation: Value = create_invitation("did-planning-poker".to_string()).await;

        let inv_did = invitation["invitation"]["services"][0]["recipientKeys"]
            .as_array()
            .unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        println!("{}", invitation);

        let duration_millis = ping(&key, inv_did, host).await.unwrap();
        assert!(duration_millis > 1);
        assert!(duration_millis < 1000);
        Ok(())
    }
}
