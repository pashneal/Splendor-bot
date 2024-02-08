mod card;
mod color;
mod token;
mod player;
mod game;
mod nobles;

fn main() {
    println!("Hello, world!");
    println!("Game: {:#?}", game::Game::new(2));
}
