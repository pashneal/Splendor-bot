use splendor_tourney::*;
use std::sync::Arc;
use tungstenite::{connect, Message};
use url::Url;

fn main() {
    env_logger::init();

    let (mut socket, response) =
        connect(Url::parse("ws://localhost:3030/game").unwrap()).expect("Can't connect");

    println!("Connected to the game server");
    loop {
        let msg = socket.read().expect("Error reading message");
        println!("Received: {}", msg);
    }
}
