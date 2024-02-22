use crate::game_logic::*;
use crate::player::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::sync::Arc;
use crate::card::Card;

pub mod protocol;

/// A module for running games across multiple clients. Can be fed binaries
/// and run them in a tournament style. The protocol for communication is
/// given by JSON messages that update the game state.
pub struct Arena {
    pub game: Game,
    pub clients: Vec<String>,
    pub timeout: Duration, 
}

/// A struct given to each client that contains all public information and private
/// information known only to that client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    board: Board,
    history: GameHistory,
    players: Vec<PlayerPublicInfo>,
    current_player: Player,
    legal_actions: Vec<Action>,
}

impl Arena {
    pub fn new(players: u8) -> Arena {
        let card_lookup = Arc::new(Card::all());
        let game = Game::new(players, card_lookup);
        let clients = Vec::new();
        let timeout = Duration::from_secs(10);
        Arena {
            game,
            clients,
            timeout,
        }
    }
    pub fn client_info(&self) -> ClientInfo {
        let players = self.game.players().iter().map(|p| p.to_public()).collect();
        let legal_actions = self
            .game
            .get_legal_actions()
            .expect("Cannot get legal actions");

        ClientInfo {
            board: Board::from_game(&self.game),
            history: self.game.history(),
            players,
            current_player: self.game.current_player(),
            legal_actions,
        }
    }
}


// Need an arena where multiple clients can compete
//     - Clients are binaries
//          -Each binary is equipped with an arg that binds them to a port  
//     - ClientInfo released to each client when it is their turn to move
//     - Clients can send their move to the arena
//          - Clients should also have a timeout (configurable in Arena)
//          - Should gracefully handle client side crashes
//          - May print logging?
//     - Arena will update the game state
//     - When the game is over, issue a special command (or just terminate the connections)
//     - Display the Winner and stats??
