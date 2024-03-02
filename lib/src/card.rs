use crate::gem_type::GemType;
use crate::token::Tokens;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone, Debug, Default, Serialize, Deserialize)]
pub struct Cost {
    onyx: i8,
    sapphire: i8,
    emerald: i8,
    ruby: i8,
    diamond: i8,
}

impl Index<GemType> for Cost {
    type Output = i8;

    fn index<'a>(&'a self, color: GemType) -> &'a i8 {
        match color {
            GemType::Onyx => &self.onyx,
            GemType::Sapphire => &self.sapphire,
            GemType::Emerald => &self.emerald,
            GemType::Ruby => &self.ruby,
            GemType::Diamond => &self.diamond,
            _ => panic!("Invalid color in Cost object"),
        }
    }
}
impl IndexMut<GemType> for Cost {
    fn index_mut<'a>(&'a mut self, color: GemType) -> &'a mut i8 {
        match color {
            GemType::Onyx => &mut self.onyx,
            GemType::Sapphire => &mut self.sapphire,
            GemType::Emerald => &mut self.emerald,
            GemType::Ruby => &mut self.ruby,
            GemType::Diamond => &mut self.diamond,
            _ => panic!("Invalid color in Cost object"),
        }
    }
}

impl Cost {
    pub fn discounted_with(&self, token: &Tokens) -> Cost {
        Cost {
            onyx: 0.max(self.onyx - token.onyx),
            sapphire: 0.max(self.sapphire - token.sapphire),
            emerald: 0.max(self.emerald - token.emerald),
            ruby: 0.max(self.ruby - token.ruby),
            diamond: 0.max(self.diamond - token.diamond),
        }
    }
    pub fn to_tokens(&self) -> Tokens {
        Tokens {
            onyx: self.onyx,
            sapphire: self.sapphire,
            emerald: self.emerald,
            ruby: self.ruby,
            diamond: self.diamond,
            gold: 0,
        }
    }
    pub fn from_tokens(tokens: &Tokens) -> Cost {
        debug_assert!(tokens.gold == 0, "Cannot convert tokens to cost with gold");
        Cost {
            onyx: tokens.onyx,
            sapphire: tokens.sapphire,
            emerald: tokens.emerald,
            ruby: tokens.ruby,
            diamond: tokens.diamond,
        }
    }
}

pub type CardId = u8;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Card {
    points: u8,
    cost: Cost,
    gem_type: GemType,
    id: CardId,
    tier: u8,
}

impl Card {
    pub fn cost(&self) -> Cost {
        self.cost
    }

    pub fn points(&self) -> u8 {
        self.points
    }

    pub fn gem(&self) -> GemType {
        self.gem_type
    }

    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn tier(&self) -> u8 {
        self.tier
    }
    pub fn all() -> Vec<Card> {
        vec![
            Card {
                id: 0,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:1,emerald:1,ruby:1),
            },
            Card {
                id: 1,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:2,emerald:1,ruby:1),
            },
            Card {
                id: 2,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(diamond:2,sapphire:2,ruby:1),
            },
            Card {
                id: 3,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(emerald:1,ruby:3,onyx:1),
            },
            Card {
                id: 4,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(emerald:2,ruby:1),
            },
            Card {
                id: 5,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(diamond:2,emerald:2),
            },
            Card {
                id: 6,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 0,
                cost: macros::cost!(emerald:3),
            },
            Card {
                id: 7,
                tier: 1,
                gem_type: GemType::Onyx,
                points: 1,
                cost: macros::cost!(sapphire:4),
            },
            Card {
                id: 8,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(diamond:1,emerald:1,ruby:1,onyx:1),
            },
            Card {
                id: 9,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(diamond:1,emerald:1,ruby:2,onyx:1),
            },
            Card {
                id: 10,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(diamond:1,emerald:2,ruby:2),
            },
            Card {
                id: 11,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(sapphire:1,emerald:3,ruby:1),
            },
            Card {
                id: 12,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(diamond:1,onyx:2),
            },
            Card {
                id: 13,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(emerald:2,onyx:2),
            },
            Card {
                id: 14,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 0,
                cost: macros::cost!(onyx:3),
            },
            Card {
                id: 15,
                tier: 1,
                gem_type: GemType::Sapphire,
                points: 1,
                cost: macros::cost!(ruby:4),
            },
            Card {
                id: 16,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(sapphire:1,emerald:1,ruby:1,onyx:1),
            },
            Card {
                id: 17,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(sapphire:1,emerald:2,ruby:1,onyx:1),
            },
            Card {
                id: 18,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(sapphire:2,emerald:2,onyx:1),
            },
            Card {
                id: 19,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(diamond:3,sapphire:1,onyx:1),
            },
            Card {
                id: 20,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(ruby:2,onyx:1),
            },
            Card {
                id: 21,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(sapphire:2,onyx:2),
            },
            Card {
                id: 22,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 0,
                cost: macros::cost!(sapphire:3),
            },
            Card {
                id: 23,
                tier: 1,
                gem_type: GemType::Diamond,
                points: 1,
                cost: macros::cost!(emerald:4),
            },
            Card {
                id: 24,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:1,ruby:1,onyx:1),
            },
            Card {
                id: 25,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:1,ruby:1,onyx:2),
            },
            Card {
                id: 26,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(sapphire:1,ruby:2,onyx:2),
            },
            Card {
                id: 27,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:3,emerald:1),
            },
            Card {
                id: 28,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(diamond:2,sapphire:1),
            },
            Card {
                id: 29,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(sapphire:2,ruby:2),
            },
            Card {
                id: 30,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 0,
                cost: macros::cost!(ruby:3),
            },
            Card {
                id: 31,
                tier: 1,
                gem_type: GemType::Emerald,
                points: 1,
                cost: macros::cost!(onyx:4),
            },
            Card {
                id: 32,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:1,sapphire:1,emerald:1,onyx:1),
            },
            Card {
                id: 33,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:2,sapphire:1,emerald:1,onyx:1),
            },
            Card {
                id: 34,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:2,emerald:1,onyx:2),
            },
            Card {
                id: 35,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:1,ruby:1,onyx:3),
            },
            Card {
                id: 36,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(sapphire:2,emerald:1),
            },
            Card {
                id: 37,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:2,ruby:2),
            },
            Card {
                id: 38,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 0,
                cost: macros::cost!(diamond:3),
            },
            Card {
                id: 39,
                tier: 1,
                gem_type: GemType::Ruby,
                points: 1,
                cost: macros::cost!(diamond:4),
            },
            Card {
                id: 40,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 1,
                cost: macros::cost!(diamond:3,sapphire:2,emerald:2),
            },
            Card {
                id: 41,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 1,
                cost: macros::cost!(diamond:3,emerald:3,onyx:2),
            },
            Card {
                id: 42,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 2,
                cost: macros::cost!(sapphire:1,emerald:4,ruby:2),
            },
            Card {
                id: 43,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 2,
                cost: macros::cost!(emerald:5,ruby:3),
            },
            Card {
                id: 44,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 2,
                cost: macros::cost!(diamond:5),
            },
            Card {
                id: 45,
                tier: 2,
                gem_type: GemType::Onyx,
                points: 3,
                cost: macros::cost!(onyx:6),
            },
            Card {
                id: 46,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 1,
                cost: macros::cost!(sapphire:2,emerald:2,ruby:3),
            },
            Card {
                id: 47,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 1,
                cost: macros::cost!(sapphire:2,emerald:3,onyx:3),
            },
            Card {
                id: 48,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 2,
                cost: macros::cost!(diamond:5,sapphire:3),
            },
            Card {
                id: 49,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 2,
                cost: macros::cost!(diamond:2,ruby:1,onyx:4),
            },
            Card {
                id: 50,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 2,
                cost: macros::cost!(sapphire:5),
            },
            Card {
                id: 51,
                tier: 2,
                gem_type: GemType::Sapphire,
                points: 3,
                cost: macros::cost!(sapphire:6),
            },
            Card {
                id: 52,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 1,
                cost: macros::cost!(emerald:3,ruby:2,onyx:2),
            },
            Card {
                id: 53,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 1,
                cost: macros::cost!(diamond:2,sapphire:3,ruby:3),
            },
            Card {
                id: 54,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 2,
                cost: macros::cost!(emerald:1,ruby:4,onyx:2),
            },
            Card {
                id: 55,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 2,
                cost: macros::cost!(ruby:5,onyx:3),
            },
            Card {
                id: 56,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 2,
                cost: macros::cost!(ruby:5),
            },
            Card {
                id: 57,
                tier: 2,
                gem_type: GemType::Diamond,
                points: 3,
                cost: macros::cost!(diamond:6),
            },
            Card {
                id: 58,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 1,
                cost: macros::cost!(diamond:3,emerald:2,ruby:3),
            },
            Card {
                id: 59,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 1,
                cost: macros::cost!(diamond:2,sapphire:3,onyx:2),
            },
            Card {
                id: 60,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 2,
                cost: macros::cost!(diamond:4,sapphire:2,onyx:1),
            },
            Card {
                id: 61,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 2,
                cost: macros::cost!(sapphire:5,emerald:3),
            },
            Card {
                id: 62,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 2,
                cost: macros::cost!(emerald:5),
            },
            Card {
                id: 63,
                tier: 2,
                gem_type: GemType::Emerald,
                points: 3,
                cost: macros::cost!(emerald:6),
            },
            Card {
                id: 64,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 1,
                cost: macros::cost!(diamond:2,ruby:2,onyx:3),
            },
            Card {
                id: 65,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 1,
                cost: macros::cost!(sapphire:3,ruby:2,onyx:3),
            },
            Card {
                id: 66,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 2,
                cost: macros::cost!(diamond:1,sapphire:4,emerald:2),
            },
            Card {
                id: 67,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 2,
                cost: macros::cost!(diamond:3,onyx:5),
            },
            Card {
                id: 68,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 2,
                cost: macros::cost!(onyx:5),
            },
            Card {
                id: 69,
                tier: 2,
                gem_type: GemType::Ruby,
                points: 3,
                cost: macros::cost!(ruby:6),
            },
            Card {
                id: 70,
                tier: 3,
                gem_type: GemType::Onyx,
                points: 3,
                cost: macros::cost!(diamond:3,sapphire:3,emerald:5,ruby:3),
            },
            Card {
                id: 71,
                tier: 3,
                gem_type: GemType::Onyx,
                points: 4,
                cost: macros::cost!(ruby:7),
            },
            Card {
                id: 72,
                tier: 3,
                gem_type: GemType::Onyx,
                points: 4,
                cost: macros::cost!(emerald:3,ruby:6,onyx:3),
            },
            Card {
                id: 73,
                tier: 3,
                gem_type: GemType::Onyx,
                points: 5,
                cost: macros::cost!(ruby:7,onyx:3),
            },
            Card {
                id: 74,
                tier: 3,
                gem_type: GemType::Sapphire,
                points: 3,
                cost: macros::cost!(diamond:3,emerald:3,ruby:3,onyx:5),
            },
            Card {
                id: 75,
                tier: 3,
                gem_type: GemType::Sapphire,
                points: 4,
                cost: macros::cost!(diamond:7),
            },
            Card {
                id: 76,
                tier: 3,
                gem_type: GemType::Sapphire,
                points: 4,
                cost: macros::cost!(diamond:6,sapphire:3,onyx:3),
            },
            Card {
                id: 77,
                tier: 3,
                gem_type: GemType::Sapphire,
                points: 5,
                cost: macros::cost!(diamond:7,sapphire:3),
            },
            Card {
                id: 78,
                tier: 3,
                gem_type: GemType::Diamond,
                points: 3,
                cost: macros::cost!(sapphire:3,emerald:3,ruby:5,onyx:3),
            },
            Card {
                id: 79,
                tier: 3,
                gem_type: GemType::Diamond,
                points: 4,
                cost: macros::cost!(onyx:7),
            },
            Card {
                id: 80,
                tier: 3,
                gem_type: GemType::Diamond,
                points: 4,
                cost: macros::cost!(diamond:3,ruby:3,onyx:6),
            },
            Card {
                id: 81,
                tier: 3,
                gem_type: GemType::Diamond,
                points: 5,
                cost: macros::cost!(diamond:3,onyx:7),
            },
            Card {
                id: 82,
                tier: 3,
                gem_type: GemType::Emerald,
                points: 3,
                cost: macros::cost!(diamond:5,sapphire:3,ruby:3,onyx:3),
            },
            Card {
                id: 83,
                tier: 3,
                gem_type: GemType::Emerald,
                points: 4,
                cost: macros::cost!(sapphire:7),
            },
            Card {
                id: 84,
                tier: 3,
                gem_type: GemType::Emerald,
                points: 4,
                cost: macros::cost!(diamond:3,sapphire:6,emerald:3),
            },
            Card {
                id: 85,
                tier: 3,
                gem_type: GemType::Emerald,
                points: 5,
                cost: macros::cost!(sapphire:7,emerald:3),
            },
            Card {
                id: 86,
                tier: 3,
                gem_type: GemType::Ruby,
                points: 3,
                cost: macros::cost!(diamond:3,sapphire:5,emerald:3,onyx:3),
            },
            Card {
                id: 87,
                tier: 3,
                gem_type: GemType::Ruby,
                points: 4,
                cost: macros::cost!(emerald:7),
            },
            Card {
                id: 88,
                tier: 3,
                gem_type: GemType::Ruby,
                points: 4,
                cost: macros::cost!(sapphire:3,emerald:6,ruby:3),
            },
            Card {
                id: 89,
                tier: 3,
                gem_type: GemType::Ruby,
                points: 5,
                cost: macros::cost!(emerald:7,ruby:3),
            },
        ]
    }
}

mod macros {
    macro_rules! cost {
        ( $($label : ident : $value : expr),* ) => {
            {
                Cost {
                    $($label : $value,)*
                    ..Default::default()
                }
            }
        };
    }
    pub(crate) use cost;
}
