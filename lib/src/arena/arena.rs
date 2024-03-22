use crate::card::Card;
use crate::game_logic::*;
use crate::player::*;
use crate::JSONable;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;

use crate::arena::protocol::*;
use crate::arena::replay::*;

/// A module for running games across multiple clients. Can be fed binaries
/// and run them in a tournament style. The protocol for communication is
/// given by JSON messages across local websockets that update the game state.
pub struct Arena {
    game: Game,
    pub clients: Vec<String>,
    pub initial_time : Duration,
    pub increment : Duration,
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
    pub fn new(players: u8, binaries: Vec<String>, initial_time: Duration, increment: Duration) -> Arena {
        let card_lookup = Arc::new(Card::all());
        let game = Game::new(players, card_lookup);
        let clients = binaries;

        Arena {
            game: game.clone(),
            replay: Either::Initialized(Replay::new(game)),
            clients,
            initial_time,
            increment,
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
            _ => None,
        }
    }

    pub fn play_action(&mut self, action : Action) {
        // TODO: adjust times for players based on increment
        self.game.play_action(action);
    }

    pub fn get_legal_actions(&self) -> Option<Vec<Action>> {
        self.game.get_legal_actions()
    }

    pub fn current_player_num(&self) -> usize {
        self.game.current_player_num()
    }

    pub fn get_winner(&self) -> Option<usize> {
        self.game.get_winner()
    }

    pub fn board(&self) -> Board {
        Board::from_game(&self.game)
    }

    pub fn players(&self) -> &Vec<Player> {
        self.game.players()
    }
}

pub struct GameResults {}
