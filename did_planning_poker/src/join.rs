// https://github.com/chriamue/did-planning-poker/blob/main/join.md
use did_key::{DIDCore, KeyPair};
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::forward::ForwardBuilder;
use didcomm_rs::Message;
use serde_json::json;

#[derive(Default)]
pub struct JoinResponseBuilder {
    session: Option<String>,
    alias: Option<String>,
    icon: Option<String>,
    did: Option<String>,
    message: Option<Message>,
}

impl JoinResponseBuilder {
    pub fn new() -> Self {
        JoinResponseBuilder {
            did: None,
            session: None,
            alias: None,
            icon: None,
            message: None,
        }
    }

    pub fn session(&mut self, session: String) -> &mut Self {
        self.session = Some(session);
        self
    }

    pub fn did(&mut self, did: String) -> &mut Self {
        self.did = Some(did);
        self
    }

    pub fn alias(&mut self, alias: String) -> &mut Self {
        self.alias = Some(alias);
        self
    }

    pub fn icon(&mut self, icon: String) -> &mut Self {
        self.icon = Some(icon);
        self
    }

    pub fn message(&mut self, message: Message) -> &mut Self {
        self.message = Some(message);
        self
    }

    pub fn build_join(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://github.com/chriamue/did-planning-poker/blob/main/join.md#join")
            .body(
                &json!(
                {"id": self.session.as_ref().unwrap(), "alias": self.alias.as_ref().unwrap(), "icon": self.icon.as_ref().unwrap(), "did": self.did.as_ref().unwrap()})
                .to_string(),
            ))
    }

    pub fn build_accept(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://github.com/chriamue/did-planning-poker/blob/main/join.md#accept")
            .thid(&self.message.as_ref().unwrap().get_didcomm_header().id))
    }

    pub fn build_reject(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://github.com/chriamue/did-planning-poker/blob/main/join.md#reject")
            .thid(&self.message.as_ref().unwrap().get_didcomm_header().id))
    }
}

pub async fn send_join(
    session: String,
    alias: String,
    icon: String,
    key: &KeyPair,
    did_to: String,
    did_mediator: String,
    host: String,
) -> Result<String, &'static str> {
    let did_doc = key.get_did_document(Default::default());
    let did_from = did_doc.id.to_string();

    let client = reqwest::Client::new();

    let request = JoinResponseBuilder::new()
        .session(session)
        .alias(alias)
        .icon(icon)
        .did(did_from.to_string())
        .build_join()
        .unwrap()
        .from(&did_from);
    let id = request.get_didcomm_header().id.to_string();
    let request = sign_and_encrypt(&request, &did_from, &did_to, key)
        .await
        .unwrap();

    let request = ForwardBuilder::new()
        .message(serde_json::to_string(&request).unwrap())
        .did(did_to)
        .build()
        .unwrap();
    let request = sign_and_encrypt(&request, &did_from, &did_mediator, key)
        .await
        .unwrap();
    let response = client
        .post(host.clone())
        .json(&request)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", response.status());
        return Err("join failed");
    }
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_join() {
        let response = JoinResponseBuilder::new()
            .session("42".to_string())
            .alias("alice".to_string())
            .icon("https://example.com/icon".to_string())
            .did("did:example".to_string())
            .build_join()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/join.md#join"
        );

        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }

    #[test]
    fn test_build_join_accept() {
        let response = JoinResponseBuilder::new()
            .session("42".to_string())
            .alias("alice".to_string())
            .icon("https://example.com/icon".to_string())
            .did("did:example".to_string())
            .build_join()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/join.md#join"
        );

        let accept = JoinResponseBuilder::new()
            .message(response)
            .build_accept()
            .unwrap();

        assert_eq!(
            accept.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/join.md#accept"
        );

        println!("{}", serde_json::to_string_pretty(&accept).unwrap());
    }

    #[test]
    fn test_build_join_reject() {
        let response = JoinResponseBuilder::new()
            .session("42".to_string())
            .alias("alice".to_string())
            .icon("https://example.com/icon".to_string())
            .did("did:example".to_string())
            .build_join()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/join.md#join"
        );

        let reject = JoinResponseBuilder::new()
            .message(response)
            .build_reject()
            .unwrap();

        assert_eq!(
            reject.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/join.md#reject"
        );

        println!("{}", serde_json::to_string_pretty(&reject).unwrap());
    }
}
