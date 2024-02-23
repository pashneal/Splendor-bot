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

use log::{debug, error, info, trace};

type Clients = Arc<RwLock<HashMap<usize, SplitSink<WebSocket, Message>>>>;
type ArenaLock = Arc<RwLock<Arena>>;

type StdError = Box<dyn std::error::Error>;

const TIMEOUT : Duration = Duration::from_secs(4);

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
    if arena.read().await.game.current_player_num() != player_id {
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

    let init_clients = clients.clone();
    let init_arena = arena.clone();

    // Convert messages from the client into a stream of actions
    // So we play them in the game as soon as they come in
    tokio::spawn( async move {
        while let Some(action_text) = client_rx.next().await {
            trace!("Received message: {:?}", action_text);
            if let Err(e) = action_text {
                error!("error reading message: {}", e);
                break;
            }
            let action = action_text.unwrap();

            let action  = parse_action(&action);
            if let Err(e) = action {
                error!("error parsing action! {:?}", e);
                break;
            }
            let action = action.unwrap();

            // Make sure user is authorized to make this action at this time
            if !validate_action(&action, my_id, arena.clone()).await {
                error!("invalid action! {:?}", action);
                break;
            }

            trace!("{} played {:?}", my_id, action);
            arena.write().await.game.play_action(action);
            action_played(clients.clone(), arena.clone()).await;
        }
        info!("{} disconnected", my_id);
        user_disconnected(my_id, clients, arena).await;
    });



    let num_players = init_arena.read().await.game.players().len();
    user_initialized(my_id, init_clients.clone(), init_arena.clone()).await;

    // All users are connected, start the game
    if my_id == num_players - 1 {
        game_initialized(init_clients, init_arena).await;
    }
}

async fn game_initialized(clients: Clients, arena: ArenaLock) {
    info!("All users locked and loaded! Game starting!");
    action_played(clients, arena).await;
}

async fn user_initialized(my_id: usize, clients: Clients, arena: ArenaLock) {
    info!("{} connected", my_id);
}

async fn user_disconnected(my_id: usize, clients: Clients, arena: ArenaLock) {
    clients.write().await.remove(&my_id);
}

async fn action_played(clients: Clients, arena: ArenaLock) {

    // Auto play for any given player if there is only 1 legal action
    loop {

        // If the game is over, don't do anything else 
        if arena.read().await.is_game_over() {
            info!("Game over!");
            info!("Winner: {:?}", arena.read().await.game.get_winner());
            return 
        }

        let actions = arena.read().await.game.get_legal_actions().expect("No legal actions!");
        if actions.len() != 1 {
            break;
        }
        let action = actions[0].clone();
        trace!("Auto played action: {:?}", action);
        arena.write().await.game.play_action(action);
    }

    trace!("Sending game state to clients...");
    // Determine which client to send the next game state to 
    let client_info = arena.read().await.client_info();
    let player_num = client_info.current_player_num;

    // Wait up to TIMEOUT for the player to come online and make a move
    if let None = clients.read().await.get(&player_num) {
        tokio::time::sleep(TIMEOUT).await;
    }

    trace!("Sending game state to player {}", player_num);
    if let Some(tx) = clients.write().await.get_mut(&player_num) {
        let info = Message::text(serde_json::to_string(&client_info).unwrap());
        tx.send(info).await.unwrap();
        trace!("Sent game state!");
    } else {
        panic!("no tx for client with id {}", player_num);
    }

}
