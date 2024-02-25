use serde::{Deserialize, Serialize};
#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum Color {
    Onyx,
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Gold,
}

impl Color {
    pub fn all_expect_gold() -> Vec<Color> {
        vec![
            Color::Onyx,
            Color::Sapphire,
            Color::Emerald,
            Color::Ruby,
            Color::Diamond,
        ]
    }
    pub fn all() -> Vec<Color> {
        vec![
            Color::Onyx,
            Color::Sapphire,
            Color::Emerald,
            Color::Ruby,
            Color::Diamond,
            Color::Gold,
        ]
    }
}
