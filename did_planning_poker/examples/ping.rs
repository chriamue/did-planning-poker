use did_key::{generate, X25519KeyPair};
use did_planning_poker::mediation::create_invitation;
use did_planning_poker::ping::ping;
use serde_json::Value;

#[tokio::main]
async fn main() {
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
    println!("PING {} @ {}", inv_did, host);
    let duration_millis = ping(&key, inv_did, host).await.unwrap();
    println!("time = {}ms", duration_millis);
}
