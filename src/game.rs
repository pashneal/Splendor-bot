use crate::player::Player;
use crate::nobles::*;
use crate::card::{CardId, Card};
use crate::token::Tokens;
use crate::color::Color;

use rand::thread_rng;
use rand::seq::SliceRandom;


#[derive(Debug, Clone)]
pub struct Game {
    players : Vec<Player>,
    tokens : Tokens,
    decks : Vec<Vec<Card>>,
    current_player : u8,
    nobles : Vec<Noble>,
}

impl Game {
    pub fn new(players: u8) -> Game {
        let mut decks = Vec::new();
        for tier in 1..=3 {
            let mut deck = Vec::new();
            for card in Card::all() {
                if card.tier() == tier {
                    deck.push(card);
                }
            }
            decks.push(deck);

        }

        let mut nobles = Noble::all();
        nobles.shuffle(&mut thread_rng());
        nobles.truncate(players as usize + 1);

        Game {
            players: (0..players).map(|_| Player::new()).collect(),
            tokens: Tokens::start(players),
            decks,
            current_player: 0,
            nobles,
        }
    }
}

pub enum Phase {
    PlayerStart, // Take some player action
    PlayerTokenCapExceeded, // [Optional] Player has > 10 tokens
    NobleAction, // See if any nobles get attracted (multiple may be attracted)
    PlayerActionEnd,  // Finish the turn and see if the round should continue
}

pub enum Action {
    TakeDouble(Color),
    TakeDistinct(Vec<Color>),
    Reserve(CardId),
    ReserveDeck(u8),
    Purchase(CardId),
    Discard(Vec<Color>),

    AttractNoble(NobleId),

    /// Marker for passing the turn to the next player
    Continue,
}
