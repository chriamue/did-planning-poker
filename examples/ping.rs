use std::env;
use did_key::{generate, DIDCore, Ed25519KeyPair};
use did_planning_poker::ping::ping;

#[tokio::main]
async fn main() {
    let key = generate::<Ed25519KeyPair>(None);
    ping(&key).await;

    let name = env::args().skip(1).next();
    println!("Hello, {}!", name.unwrap_or("world".into()));
}