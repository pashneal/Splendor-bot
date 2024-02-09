mod card;
mod color;
mod token;
mod player;
mod game;
mod nobles;

use std::sync::Arc;
use card::Card;

fn main() {
    println!("Hello, world!");
    let card_lookup : Arc<Vec<Card>> = Arc::new(card::Card::all());
    println!("Game: {:#?}", game::Game::new(2, card_lookup));
}
