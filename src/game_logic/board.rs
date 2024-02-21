use super::*;
use serde::{Deserialize, Serialize};

/// Contains public board information that all players have
/// access to such as card counts, nobles available, and gems available in
/// the piles. Removes any hidden information (such as the order that cards
/// will be drawn from the deck).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Board {
    deck_counts: [usize; 3],
    available_cards: Vec<Vec<CardId>>,
    nobles: Vec<NobleId>,
    tokens: Tokens,
}

impl Board {
    pub fn from_game(game: &Game) -> Self {
        let mut available_cards = vec![vec![]; 3];
        for card in game.cards() {
            available_cards[card.tier() as usize].push(card.id());
        }
        Board {
            deck_counts: game.deck_counts(),
            available_cards,
            nobles: game.nobles().iter().map(|n| n.id()).collect(),
            tokens: game.tokens().clone(),
        }
    }
}
