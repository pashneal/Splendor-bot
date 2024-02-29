use serde::{Deserialize, Serialize};
#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash, Serialize, Deserialize)]
pub enum GemType {
    Onyx,
    Sapphire,
    Emerald,
    Ruby,
    Diamond,
    Gold,
}

impl GemType {
    pub fn all_expect_gold() -> Vec<GemType> {
        vec![
            GemType::Onyx,
            GemType::Sapphire,
            GemType::Emerald,
            GemType::Ruby,
            GemType::Diamond,
        ]
    }
    pub fn all() -> Vec<GemType> {
        vec![
            GemType::Onyx,
            GemType::Sapphire,
            GemType::Emerald,
            GemType::Ruby,
            GemType::Diamond,
            GemType::Gold,
        ]
    }
}
