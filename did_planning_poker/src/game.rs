// https://github.com/chriamue/did-planning-poker/blob/main/join.md
use did_key::KeyPair;
use didcomm_mediator::message::sign_and_encrypt;
use didcomm_mediator::protocols::forward::ForwardBuilder;
use didcomm_rs::Message;
use serde::{Deserialize, Serialize};
use serde_json::json;

#[derive(Default, Serialize, Deserialize)]
pub struct Player {
    did: String,
    alias: String,
    ping: f32,
    voted: String,
}

#[derive(Default)]
pub struct GameResponseBuilder {
    id: Option<String>,
    players: Option<Vec<Player>>,
}

impl GameResponseBuilder {
    pub fn new() -> Self {
        GameResponseBuilder {
            id: None,
            players: None,
        }
    }

    pub fn id(&mut self, id: String) -> &mut Self {
        self.id = Some(id);
        self
    }
    pub fn players(&mut self, players: Vec<Player>) -> &mut Self {
        self.players = Some(players);
        self
    }

    pub fn build_players(&mut self) -> Result<Message, &'static str> {
        Ok(Message::new()
            .m_type("https://github.com/chriamue/did-planning-poker/blob/main/game.md#players")
            .body(
                &json!(
                {"id": self.id.as_ref().unwrap(), "players": self.players.as_ref().unwrap()})
                .to_string(),
            ))
    }
}

pub async fn send_players(
    id: String,
    players: Vec<Player>,
    key: &KeyPair,
    did_to: String,
    did_mediator: String,
    host: String,
) -> Result<String, &'static str> {
    let client = reqwest::Client::new();

    let request = GameResponseBuilder::new()
        .id(id)
        .players(players)
        .build_players()
        .unwrap();
    let id = request.get_didcomm_header().id.to_string();
    let request = sign_and_encrypt(&request, &did_to, key);

    let request = ForwardBuilder::new()
        .message(serde_json::to_string(&request).unwrap())
        .did(did_to)
        .build()
        .unwrap();
    let request = sign_and_encrypt(&request, &did_mediator, key);
    let response = client
        .post(host.clone())
        .json(&request)
        .send()
        .await
        .unwrap();

    if !response.status().is_success() {
        println!("{:?}", response.status());
        return Err("players failed");
    }
    Ok(id)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_build_players() {
        let response = GameResponseBuilder::new()
            .id("42".to_string())
            .players(Vec::new())
            .build_players()
            .unwrap();

        assert_eq!(
            response.get_didcomm_header().m_type,
            "https://github.com/chriamue/did-planning-poker/blob/main/game.md#players"
        );

        println!("{}", serde_json::to_string_pretty(&response).unwrap());
    }
}