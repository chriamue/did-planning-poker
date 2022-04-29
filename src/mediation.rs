use did_key::{generate, DIDCore, Ed25519KeyPair};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use tokio_tungstenite::tungstenite::{connect, Message};
use url::Url;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediationRequest {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub m_type: String,
}

impl Default for MediationRequest {
    fn default() -> Self {
        MediationRequest {
            id: Uuid::new_v4().to_string(),
            m_type: "https://didcomm.org/coordinate-mediation/1.0/mediate-request".to_string(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Connection {
    #[serde(rename = "DID")]
    did: String,
    #[serde(rename = "DIDDoc")]
    did_doc: Value,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ConnectionRequest {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub m_type: String,
    pub label: String,
    pub connection: Connection,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CreateInvitation {
    pub label: String,
}

impl Default for ConnectionRequest {
    fn default() -> Self {
        use did_key::*;
        let key = generate::<Ed25519KeyPair>(None);
        let did_doc = key.get_did_document(Config::default());
        let did = did_doc.id.clone();

        ConnectionRequest {
            id: Uuid::new_v4().to_string(),
            m_type: "https://didcomm.org/connections/1.0/request".to_string(),
            label: "did-planning-poker".to_string(),
            connection: Connection {
                did,
                did_doc: serde_json::to_value(did_doc).unwrap(),
            },
        }
    }
}

pub async fn create_invitation(label: String) -> Value {
    let request = CreateInvitation { label: label };
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/outofband/create-invitation")
        .json(&request)
        .send()
        .await
        .unwrap();
    res.json().await.unwrap()
}

pub async fn accept_invitation(label: String, invitation: Value) -> String {
    let mut invitation = invitation.clone();
    invitation["my_label"] = label.into();
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/outofband/accept-invitation")
        .json(&invitation)
        .send()
        .await
        .unwrap();
    let accept_response: Value = res.json().await.unwrap();
    let connection_id = accept_response["connection_id"].as_str().unwrap();
    connection_id.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use didcomm_rs::Error;
    use std::{thread, time};

    #[ignore = "not now"]
    #[tokio::test]
    async fn send_create_invitation() -> Result<(), Error> {
        let invitation: Value = create_invitation("did-planning-poker".to_string()).await;
        let connection_id = accept_invitation("user".to_string(), invitation).await;

        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "http://localhost:8080/connections/{}/accept-request",
                connection_id
            ))
            .send()
            .await
            .unwrap();
        println!("{}", res.text().await.unwrap());

        Ok(())
    }

    #[ignore = "not now"]
    #[tokio::test]
    async fn send_mediation_request() -> Result<(), Error> {
        let invitation: Value = create_invitation("did-planning-poker".to_string()).await;
        let connection_id = accept_invitation("user".to_string(), invitation).await;
        let mut request = MediationRequest::default();
        request.id = connection_id;
        println!("{}", serde_json::to_string(&request).unwrap());

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8081/")
            .json(&request)
            .send()
            .await
            .unwrap();
        println!("{}", res.text().await.unwrap());

        Ok(())
    }

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_connection_request() -> Result<(), Error> {
        let mut invitation: Value = create_invitation("did-planning-poker".to_string()).await;
        invitation["my_label"] = "user".to_string().into();
        let connection_id = accept_invitation("user".to_string(), invitation.clone()).await;

        let mut request = ConnectionRequest::default();

        let mut request = MediationRequest::default();
        request.id = connection_id;

        let (mut socket, response) =
            connect(Url::parse("ws://localhost:8082/").unwrap()).expect("Can't connect");

        println!("Connected to the server");
        println!("Response HTTP code: {}", response.status());
        println!("Response contains the following headers:");
        for (ref header, _value) in response.headers() {
            println!("* {}", header);
        }
        let json = serde_json::to_string(&request.clone()).unwrap();
        //println!("{}", json);

        socket
            .write_message(Message::Text(
                json,
            ))
            .unwrap();

        let _handler = thread::spawn(move || {
            if socket.can_read() {
                let msg = socket.read_message().expect("Error reading message");
                println!("Received: {}", msg);
            }
        });

        thread::sleep(time::Duration::from_millis(2000));

        Ok(())
    }
}
