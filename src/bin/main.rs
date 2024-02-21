use splendor_tourney::*;
use std::sync::Arc;

fn main() {
    env_logger::init();

    println!("Hello, world!");
    let card_lookup: Arc<Vec<card::Card>> = Arc::new(card::Card::all());
    println!("Game: {:#?}", game_logic::game::Game::new(2, card_lookup));
}
