use did_key::*;
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::forward::ForwardBuilder;
use didcomm_mediator::protocols::messagepickup::MessagePickupResponseBuilder;
use didcomm_mediator::protocols::trustping::TrustPingResponseBuilder;
use didcomm_rs::Message;
use instant::Instant;

pub async fn send_ping(
    key: &KeyPair,
    did_to: String,
    did_mediator: String,
    host: String,
) -> Result<String, &'static str> {
    let did_doc = key.get_did_document(Default::default());
    let did_from = did_doc.id.to_string();

    let client = reqwest::Client::new();

    let request = TrustPingResponseBuilder::new()
        .build()
        .unwrap()
        .from(&did_from);

    let id = request.get_didcomm_header().id.to_string();
    let request = sign_and_encrypt(&request, &did_to, key).unwrap();

    let request = ForwardBuilder::new()
        .message(serde_json::to_string(&request).unwrap())
        .did(did_to)
        .build()
        .unwrap();
    let request = sign_and_encrypt(&request, &did_mediator, key).unwrap();
    let response = client
        .post(host.clone())
        .json(&request)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", response.status());
        return Err("ping failed");
    }
    Ok(id)
}

pub async fn send_pong(
    thid: String,
    key: &KeyPair,
    did_to: String,
    did_mediator: String,
    host: String,
) -> Result<String, &'static str> {
    let did_doc = key.get_did_document(Default::default());
    let did_from = did_doc.id.to_string();

    let client = reqwest::Client::new();

    let request = TrustPingResponseBuilder::new()
        .thid(thid)
        .build_response()
        .unwrap()
        .from(&did_from);

    let id = request.get_didcomm_header().id.to_string();
    let request = sign_and_encrypt(&request, &did_to, key).unwrap();

    let request = ForwardBuilder::new()
        .message(serde_json::to_string(&request).unwrap())
        .did(did_to)
        .build()
        .unwrap();
    let request = sign_and_encrypt(&request, &did_mediator, key).unwrap();
    let response = client
        .post(host.clone())
        .json(&request)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", response.status());
        return Err("pong failed");
    }
    Ok(id)
}

pub async fn ping(key: &KeyPair, did_to: String, host: String) -> Result<u32, &'static str> {
    let did_doc = key.get_did_document(Default::default());
    let did_from = did_doc.id.to_string();

    let start = Instant::now();
    let client = reqwest::Client::new();

    let mut pong = false;

    let request = TrustPingResponseBuilder::new()
        .build()
        .unwrap()
        .from(&did_from);

    let thid = request.get_didcomm_header().id.clone();

    let request = sign_and_encrypt(&request, &did_to, key).unwrap();

    let response = client
        .post(host.clone())
        .json(&request)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", response.status());
        return Err("ping failed");
    }

    let request = MessagePickupResponseBuilder::new()
        .batch_size(2)
        .build_batch_pickup()
        .unwrap()
        .from(&did_from);

    let request = sign_and_encrypt(&request, &did_to, key).unwrap();

    let response = client.post(host).json(&request).send().await.unwrap();

    if !response.status().is_success() {
        return Err("ping pickup failed");
    }

    let response_json = response.text().await.unwrap();
    let received = Message::receive(&response_json, Some(&key.private_key_bytes()), None, None);

    assert!(&received.is_ok());
    let message: Message = received.unwrap();

    for attachment in message.get_attachments() {
        let response_json = attachment.data.json.as_ref().unwrap();
        let received =
            Message::receive(response_json, Some(&key.private_key_bytes()), None, None).unwrap();
        if received.get_didcomm_header().thid == Some(thid.to_string()) {
            pong = true;
        }
    }

    if !pong {
        return Err("no pong for pickup");
    }

    let duration = start.elapsed();

    Ok(duration.as_millis() as u32)
}

#[cfg(feature = "bin")]
#[cfg(test)]
mod tests {
    use super::*;
    use crate::mediation::create_invitation;
    use didcomm_rs::Error;
    use serde_json::Value;

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_ping() -> Result<(), Error> {
        let host = "https://mediator.ssi.quest";
        let key = generate::<X25519KeyPair>(None);

        let invitation: Value =
            create_invitation("did-planning-poker".to_string(), host.to_string()).await;

        let inv_did = invitation["invitation"]["services"][0]["recipientKeys"]
            .as_array()
            .unwrap()[0]
            .as_str()
            .unwrap()
            .to_string();
        println!("{}", invitation);

        let duration_millis = ping(&key, inv_did, format!("{}/didcomm", host))
            .await
            .unwrap();
        println!("PONG {} ms", duration_millis);
        assert!(duration_millis > 1);
        assert!(duration_millis < 1000);
        Ok(())
    }
}
