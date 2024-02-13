#[derive(PartialEq, Copy, Clone, Debug, Eq, Hash)]
pub enum Color {
    Black,
    Blue,
    Green,
    Red,
    White,
    Gold,
}

impl Color {
    pub fn all() -> Vec<Color> {
        vec![
            Color::Black,
            Color::Blue,
            Color::Green,
            Color::Red,
            Color::White,
            Color::Gold,
        ]
    }
}
