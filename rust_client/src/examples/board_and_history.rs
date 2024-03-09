use crate::stubs::*;

#[derive(Debug, Default)]
pub struct Bot {
}

impl Runnable<GameInfo, Action> for Bot {
    fn initialize(&mut self, _log: &mut Log) {
        // Initialize your bot here!
    }

    fn take_action(&mut self, info: GameInfo, log: &mut Log) -> Action {

        // There's lots of useful information in the GameInfo object!
        let _num_players = info.num_players;
        log.send("[board_and_history] hello from the bot");

        // Game information about the current state and the 
        // history of the game until the current point can be accessed
        // like so:

        // You can iterate over the previous states of the game
        let history = &info.history;
        let all_turns = &history.turns;
        let last_5_turns = &all_turns[all_turns.len() - 5..];

        for (player_index, actions) in last_5_turns.iter() {
            for action in actions {
                if matches!(action, Action::AttractNoble(_)) {
                    let warning = format!("[board_and_history] player {} attracted a noble", player_index);
                    let advice = "[board_and_history] maybe they're close to winning?";
                    log.send(&warning);
                    log.send(advice);
                }
            }
        }

         
        // You can also see the current state of the game by accessing the board
        let board = &info.board;
        // The board has the following attributes:
        board.deck_counts[0]; // The number of tier 0 cards left in the deck
        board.deck_counts[1]; // The number of tier 1 cards left in the deck
        board.deck_counts[2]; // The number of tier 2 cards left in the deck
        
        let _nobles = board.nobles.clone(); // The list of noble ids that are still available to grab
        let _noble_0 = Noble::from_id(0); 
        let _noble_1 = Noble::from_id(1); 
        let _noble_5 = Noble::from_id(5); 

        // You can see how many of what type of gem is left in the bank
        board.gems.onyx;
        board.gems.gold;

        // You can also see the current state of the game by accessing the players
        let players = &info.players;

        let _ = players[0].reserved_cards.clone(); // The cards that player 0 has reserved
        // Which may be None if you are not able to see the player's reserved cards

        // You can also access various properties of each player
        let my_player = &info.me();
        my_player.gems; // The number of gems the player has
        my_player.total_points; // The number of points the player has (including from nobles)
        my_player.developments; // The cards the player has played in front of them
        my_player.num_reserved_cards; // The number of cards the player currently has reserved
        

        let legal_actions = info.legal_actions;
        legal_actions[0].clone()
    }
}
