mod card;
mod color;
mod game;
mod nobles;
mod player;
mod token;

use card::Card;
use std::sync::Arc;

fn main() {
    println!("Hello, world!");
    let card_lookup: Arc<Vec<Card>> = Arc::new(card::Card::all());
    println!("Game: {:#?}", game::Game::new(2, card_lookup));
}
