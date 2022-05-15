use serde::{Deserialize, Serialize};
use serde_json::Value;

use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct MediationRequest {
    #[serde(rename = "@id")]
    pub id: String,
    #[serde(rename = "@type")]
    pub m_type: String,
    pub typ: String,
}

impl Default for MediationRequest {
    fn default() -> Self {
        MediationRequest {
            id: Uuid::new_v4().to_string(),
            m_type: "https://didcomm.org/coordinate-mediation/1.0/mediate-request".to_string(),
            typ: "application/didcomm-plain+json".to_string(),
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
        .post("http://localhost:8000/outofband/create-invitation")
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
    use url::Url;

    //#[ignore = "not now"]
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

    //#[ignore = "not now"]
    #[tokio::test]
    async fn send_mediation_request() -> Result<(), Error> {
        let invitation: Value = create_invitation("did-planning-poker".to_string()).await;
        let connection_id = accept_invitation("user".to_string(), invitation).await;
        let mut request = MediationRequest::default();
        request.id = connection_id;
        let json = serde_json::to_string(&request).unwrap();
        println!("{}", json);

        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:8081/")
            /*.header(
                CONTENT_TYPE,
                "application/didcomm-plain+json",
            )
            .header(
                ACCEPT,
                "application/json",
            )*/
            .json(&request)
            //.body(json)
            .send()
            .await
            .unwrap();
        println!("response: {}", res.text().await.unwrap());

        Ok(())
    }

}
