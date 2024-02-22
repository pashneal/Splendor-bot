use super::*;
use std::collections::HashMap;
use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};

use futures_util::{SinkExt, StreamExt, TryFutureExt, stream::SplitSink};
use tokio::sync::{mpsc, RwLock};
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::ws::{Message, WebSocket};
use derive_more::{Display, Error};
use warp::Filter;

type Clients = Arc<RwLock<HashMap<usize, SplitSink<WebSocket, Message>>>>;
type ArenaLock = Arc<RwLock<Arena>>;

type StdError = Box<dyn std::error::Error>;

static CLIENT_ID : AtomicUsize = AtomicUsize::new(0);



impl Arena {
    pub async fn launch(port : u16) {
        let arena = Arena::new(2);
        // Keep track of the game state
        let arena = Arc::new(RwLock::new(arena));
        // Turn our arena state into a new Filter
        let arena = warp::any().map(move || arena.clone());

        // Keep track of all connected players, key is usize, value
        // is a websocket sender.
        let clients = Clients::default();
        // Turn our "clients" state into a new Filter...
        let clients = warp::any().map(move || clients.clone());

        let game = warp::path("game")
            .and(warp::ws())
            .and(clients)
            .and(arena)
            .map(|ws: warp::ws::Ws, clients, arena| {
                ws.on_upgrade(move |socket| user_connected(socket, clients, arena))
            });

        let routes = game;

        // Start the server on localhost at the specified port
        warp::serve(routes).run(([127, 0, 0, 1], port)).await;
    }
}

#[derive(Debug, Display, Error)]
pub enum ParseError {
    #[display(fmt = "Unknown")]
    Unknown,
    #[display(fmt = "Cannot convert action to string")]
    CannotConvertToString,
    #[display(fmt = "Cannot convert action_string to action")]
    CannotConvertToAction,

}

fn parse_action(action_text: &Message) -> Result<Action, ParseError> {
    let action_str = action_text.to_str().map_err(|_| ParseError::CannotConvertToString)?;
    let action: Action= serde_json::from_str(action_str).map_err(|_| ParseError::CannotConvertToAction)?;
    Ok(action)
}

async fn validate_action( action: &Action, player_id : usize , arena : ArenaLock) -> bool {
    // -> Is a legal action
    let actions = arena.read().await.game.get_legal_actions();
    if actions.is_none() {
        return false;
    }
    let actions = actions.unwrap();
    if !actions.contains(action) {
        return false;
    }

    // -> Is the correct player's turn
    if (arena.read().await.game.current_player_num() != player_id) {
        return false;
    }

    // -> Is not timed out TODO

    true
}

/// Setup a new client to play the game 
async fn user_connected(ws: WebSocket, clients: Clients, arena: ArenaLock) {
    let (client_tx, mut client_rx) = ws.split();
    let my_id = CLIENT_ID.fetch_add(1, Ordering::Relaxed);
    clients.write().await.insert(my_id , client_tx);

    // Convert messages from the client into a stream of actions
    tokio::spawn( async move {
        while let Some(action_text) = client_rx.next().await {
            if let Err(e) = action_text {
                eprintln!("error reading message: {}", e);
                break;
            }
            let action = action_text.unwrap();

            let action  = parse_action(&action);
            if let Err(e) = action {
                eprintln!("error parsing action! {:?}", e);
                break;
            }
            let action = action.unwrap();

            if !validate_action(&action, my_id, arena.clone()).await {
                eprintln!("invalid action! {:?}", action);
                break;
            }

            let mut arena = arena.write().await;
            arena.game.play_action(action);
        }
        println!("{} disconnected", my_id);
        user_disconnected(my_id, &clients, arena.clone()).await;
    });

}

async fn user_disconnected(my_id: usize, clients: &Clients, arena: ArenaLock) {
}

