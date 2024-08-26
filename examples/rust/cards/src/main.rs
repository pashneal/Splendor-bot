pub use splendor_tourney::stubs::rust as stubs;

use stubs::*;
use rand::{seq::SliceRandom, thread_rng};

/// Your bot struct, which will live for the duration of the game
/// Feel free to add any fields you need, but they must implement Default!
#[derive(Debug, Default)]
pub struct Bot {
    pub name: String,
    pub turn_counter: usize,
}

impl Runnable<GameInfo, Action> for Bot {
    fn initialize(&mut self, log: &mut Log) {
        log.send("[cards.rs] Welcome to the explanation of cards!");

        // Many types of actions require interaction with some card
        // You can get a list of all the possible cards in the game with:
        let _cards = Card::all();

        // You can initialize a new card with just the ID of a card
        let card_61 = Card::from_id(61);
        let _card_42 = Card::from_id(42);

        // Actions that require a card should be created using card_id instead
        let _purchase = Action::Purchase(
            card_61.id,
            Gems {
                onyx: 1,
                sapphire: 1,
                emerald: 3,
                ruby: 1,
                diamond: 1,
                gold: 0,
            },
        );

        let _reserve_face_up = Action::ReserveFaceUp(46);

        // There are also several useful properties of a card to check out
        let card = Card::from_id(23);

        log.send(&format!("[cards.rs] ID {}", card.id));
        log.send(&format!("[cards.rs] TIER {}", card.tier));
        log.send(&format!("[cards.rs] POINTS {}", card.points));
        log.send(&format!("[cards.rs] GEM {:?}", card.gem));
        log.send(&format!("[cards.rs] COST {:?}", card.cost));
    }

    fn take_action(&mut self, info: GameInfo, log: &mut Log) -> Action {
        let legal_actions = &info.legal_actions;

        // You can grab all the current faceup cards from the game_info
        let _all_cards = info.board.all_face_up_cards();

        // Or just the ones in a specific tier
        let _tier_2_cards = info.board.face_up_cards(2);
        let _tier_1_cards = info.board.face_up_cards(1);
        let _tier_0_cards = info.board.face_up_cards(0);

        let _developments = info.me().developments;

        // You can check out the cards you have reserved
        let reserved_cards = info.me().reserved_cards.clone().unwrap();
        // Or the gems of the cards you have played
        let developments = info.me().developments;

        let mut total_developments = 0;

        for gem in Gem::all_expect_gold() {
            total_developments += developments[gem];
        }

        let message0 = format!("[cards.rs] You have {} developments", total_developments);
        let message1 = format!(
            "[cards.rs] You have {} reserved cards",
            reserved_cards.len()
        );
        log.send(&message0);
        log.send(&message1);

        // You can always tell how many cards any single player has reserved
        info.players[0].num_reserved_cards;
        info.players[1].num_reserved_cards;

        let my_index = info.me().index;
        let num_players = info.players.len();
        let not_my_index = (my_index + 1) % num_players;
        let player = &info.players[not_my_index];
        match &player.reserved_cards {
            Some(_) => {
                panic!("[cards.rs] This should not happen!!!!!");
            }
            None => {
                log.send("[cards.rs] You cannot see this player's cards as expected");
            }
        }

        let random_action = legal_actions.choose(&mut thread_rng()).unwrap();
        random_action.clone()
    }
}

fn main() {
    run_bot::<_, _, Bot>();
}
