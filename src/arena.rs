use crate::game_logic::*;
use crate::player::*;

/// A module for running games across multiple clients. Can be fed binaries 
/// and run them in a tournament style. The protocol for communication is 
/// given by JSON messages that update the game state.
pub struct Arena {
    pub game: Game,
    pub clients: Vec<String>, 
}


/// A struct given to each client that contains all public information and private
/// information known only to that client.
pub struct ClientInfo {
    board: Board,
    history: GameHistory,
    players: Vec<PlayerPublicInfo>,
    current_player: Player,
    legal_actions: Vec<Action>,
}

impl Arena {
    pub fn client_info(&self) -> ClientInfo {

        let players = self.game.players().iter().map(|p| p.to_public()).collect();
        let legal_actions = self.game.get_legal_actions().expect("Cannot get legal actions");

        ClientInfo {
            board : Board::from_game(&self.game),
            history: self.game.history(),
            players,
            current_player: self.game.current_player(),
            legal_actions,
        }
    }
}


// Need an arena where multiple clients can compete
//     - ClientInfo release to each client when it is their turn to move

