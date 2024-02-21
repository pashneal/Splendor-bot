mod game_logic;
mod card;
mod color;
mod arena;
mod nobles;
mod player;
mod token;

use card::Card;
use std::sync::Arc;

fn main() {
    env_logger::init();

    println!("Hello, world!");
    let card_lookup: Arc<Vec<Card>> = Arc::new(card::Card::all());
    println!("Game: {:#?}", game_logic::game::Game::new(2, card_lookup));
}
