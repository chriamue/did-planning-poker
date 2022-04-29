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

#[cfg(test)]
mod tests {
    use super::*;
    use didcomm_rs::Error;

    #[tokio::test]
    async fn send_create_invitation() -> Result<(), Error> {
        let request = CreateInvitation{
            label: "did-planning-poker".to_string()
        };
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8080/outofband/create-invitation")
            .json(&request)
            .send()
            .await
            .unwrap();
        let mut invitation: Value = res.json().await.unwrap();
        invitation["my_label"] = "user".to_string().into();

        let res = client
            .post("http://localhost:8080/outofband/accept-invitation")
            .json(&invitation)
            .send()
            .await
            .unwrap();
        println!("{}", res.text().await.unwrap());

        Ok(())
    }
}
