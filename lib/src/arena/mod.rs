use crate::game_logic::*;
use crate::player::*;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use std::sync::Arc;
use tokio::sync::RwLock;
use crate::JSONable;
use crate::card::Card;

pub mod protocol;
pub mod replay;

use replay::*;
pub use protocol::*;

/// A module for running games across multiple clients. Can be fed binaries
/// and run them in a tournament style. The protocol for communication is
/// given by JSON messages across local websockets that update the game state.
pub struct Arena {
    pub game: Game,
    pub clients: Vec<String>,
    pub timeout: Duration, 
    replay: Either<Replay<Initialized>, FinalizedReplay>,
}

/// A struct given to each client that contains all public information and private
/// information known only to that client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInfo {
    pub board: Board,
    pub history: GameHistory,
    pub players: Vec<PlayerPublicInfo>,
    pub current_player: Player,
    pub current_player_num: usize,
    pub legal_actions: Vec<Action>,
}

impl JSONable for ClientInfo {}

impl Arena {
    pub fn new(players: u8, binaries : Vec<String>) -> Arena {
        let card_lookup = Arc::new(Card::all());
        let game = Game::new(players, card_lookup);
        let clients = binaries;
        let timeout = Duration::from_secs(10); 

        Arena {
            game: game.clone(),
            replay: Either::Initialized(Replay::new(game)),
            clients,
            timeout,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.game.game_over()
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
            current_player_num: self.game.current_player_num(),
            legal_actions,
        }
    }

    pub fn finalize_game(&mut self) {
        let replay = self.replay.clone();
        match replay {
            Either::Initialized(replay) => {
                let history = self.game.history();
                let replay = replay.finalize_with(history);
                let replay = Arc::new(RwLock::new(replay));
                self.replay = Either::Finalized(replay);
            }
            _ => panic!("Cannot finalize game that is already finalized"),
        }
    }

    pub fn get_replay(&self) -> Option<FinalizedReplay> {
        match &self.replay {
            Either::Finalized(replay) => Some(replay.clone()),
            _ => None
        }
    }
}


pub struct GameResults {
}

// Need an arena where multiple clients can compete
//     - When the game is over, issue a special command (or just terminate the connections)
//
//     Visualization (high)
//        - auto launch visualization server after run_game executes
//        - indexable history 
//        - forward + next buttons
//     UI (medium)
//      - Colorblind friendly shapes (must)
//      - Elo
//      - arbitrary tourney? Login?
//     Servers spinup (medium-high)
//      - Git management
//      - separate
//     Action Explanation (high)
//     Sandboxing?
