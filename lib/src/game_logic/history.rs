use super::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
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

    // A move is defined a bit weirdly here,
    // it's all the actions taken by a single player in a turn
    pub fn num_moves(&self) -> i32 {
        let mut moves = 0;
        // Group the actions by player and count every transition
        self.history.iter().fold( None, |acc, (p, _)| {
            if let Some(last_p) = acc {
                if last_p != *p {
                    moves += 1;
                }
            }
            Some(*p)
        });
        moves
    }

    pub fn take_until_move(&self, move_index: i32) -> GameHistory {
        let mut new_history = vec![];
        let num_moves = self.num_moves();
        let mut move_index = 0;
        let mut last_player = None;
        for (player_num, action) in self.history.iter() {
            new_history.push((*player_num, action.clone()));
            if last_player != Some(*player_num) {
                move_index += 1;
            }
            last_player = Some(*player_num);
        }

        GameHistory::from(new_history)
    }
}

impl IntoIterator for GameHistory {
    type Item = (usize, Action);
    type IntoIter = std::vec::IntoIter<(usize, Action)>;

    fn into_iter(self) -> Self::IntoIter {
        self.history.into_iter()
    }
}
