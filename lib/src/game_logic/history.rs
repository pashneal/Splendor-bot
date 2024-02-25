use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GameHistory {
    pub history: Vec<(usize, Action)>,
}

impl GameHistory {
    pub fn new() -> Self {
        GameHistory {
            history: Vec::new(),
        }
    }

    pub fn from(history: Vec<(usize, Action)>) -> Self {
        GameHistory { history }
    }

    fn undo(&mut self) {
        self.history.pop();
    }

    fn history_since_player(&self, player_num: usize) -> GameHistory {
        let mut new_history = Vec::new();
        for (p, a) in self.history.iter().rev() {
            if *p == player_num {
                new_history.push((*p, a.clone()));
            } else {
                break;
            }
        }
        new_history.reverse();
        GameHistory::from(new_history)
    }

    pub fn add(&mut self, player_num: usize, action: Action) {
        self.history.push((player_num, action));
    }
}

impl IntoIterator for GameHistory {
    type Item = (usize, Action);
    type IntoIter = std::vec::IntoIter<(usize, Action)>;

    fn into_iter(self) -> Self::IntoIter {
        self.history.into_iter()
    }
}
