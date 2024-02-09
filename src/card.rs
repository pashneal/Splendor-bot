use crate::color::Color;
use crate::token::Tokens;
use std::ops::{Index, IndexMut};

#[derive(PartialEq, Copy, Clone, Debug, Default)]
pub struct Cost {
    black: i8,
    blue: i8,
    green: i8,
    red: i8,
    white: i8,
}

impl Index<Color> for Cost {
    type Output = i8;

    fn index<'a>(&'a self, color: Color) -> &'a i8 {
        match color {
            Color::Black => &self.black,
            Color::Blue => &self.blue,
            Color::Green => &self.green,
            Color::Red => &self.red,
            Color::White => &self.white,
            _ => panic!("Invalid color in Cost object"),
        }
    }
}
impl IndexMut<Color> for Cost {
    fn index_mut<'a>(&'a mut self, color: Color) -> &'a mut i8 {
        match color {
            Color::Black => &mut self.black,
            Color::Blue => &mut self.blue,
            Color::Green => &mut self.green,
            Color::Red => &mut self.red,
            Color::White => &mut self.white,
            _ => panic!("Invalid color in Cost object"),
        }
    }
}

impl Cost {
    pub fn discounted_with(&self, token: &Tokens) -> Cost {
        Cost {
            black: 0.max(self.black - token.black),
            blue: 0.max(self.blue - token.blue),
            green: 0.max(self.green - token.green),
            red: 0.max(self.red - token.red),
            white: 0.max(self.white - token.white),
        }
    }
    pub fn to_tokens(&self) -> Tokens {
        Tokens {
            black: self.black,
            blue: self.blue,
            green: self.green,
            red: self.red,
            white: self.white,
            gold: 0,
        }
    }
}

pub type CardId = u8;

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Card {
    points: u8,
    cost: Cost,
    color: Color,
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

    pub fn color(&self) -> Color {
        self.color
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
                color: Color::Black,
                points: 0,
                cost: macros::cost!(white:1,blue:1,green:1,red:1),
            },
            Card {
                id: 1,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(white:1,blue:2,green:1,red:1),
            },
            Card {
                id: 2,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(white:2,blue:2,red:1),
            },
            Card {
                id: 3,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(green:1,red:3,black:1),
            },
            Card {
                id: 4,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(green:2,red:1),
            },
            Card {
                id: 5,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(white:2,green:2),
            },
            Card {
                id: 6,
                tier: 1,
                color: Color::Black,
                points: 0,
                cost: macros::cost!(green:3),
            },
            Card {
                id: 7,
                tier: 1,
                color: Color::Black,
                points: 1,
                cost: macros::cost!(blue:4),
            },
            Card {
                id: 8,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(white:1,green:1,red:1,black:1),
            },
            Card {
                id: 9,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(white:1,green:1,red:2,black:1),
            },
            Card {
                id: 10,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(white:1,green:2,red:2),
            },
            Card {
                id: 11,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(blue:1,green:3,red:1),
            },
            Card {
                id: 12,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(white:1,black:2),
            },
            Card {
                id: 13,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(green:2,black:2),
            },
            Card {
                id: 14,
                tier: 1,
                color: Color::Blue,
                points: 0,
                cost: macros::cost!(black:3),
            },
            Card {
                id: 15,
                tier: 1,
                color: Color::Blue,
                points: 1,
                cost: macros::cost!(red:4),
            },
            Card {
                id: 16,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(blue:1,green:1,red:1,black:1),
            },
            Card {
                id: 17,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(blue:1,green:2,red:1,black:1),
            },
            Card {
                id: 18,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(blue:2,green:2,black:1),
            },
            Card {
                id: 19,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(white:3,blue:1,black:1),
            },
            Card {
                id: 20,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(red:2,black:1),
            },
            Card {
                id: 21,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(blue:2,black:2),
            },
            Card {
                id: 22,
                tier: 1,
                color: Color::White,
                points: 0,
                cost: macros::cost!(blue:3),
            },
            Card {
                id: 23,
                tier: 1,
                color: Color::White,
                points: 1,
                cost: macros::cost!(green:4),
            },
            Card {
                id: 24,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(white:1,blue:1,red:1,black:1),
            },
            Card {
                id: 25,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(white:1,blue:1,red:1,black:2),
            },
            Card {
                id: 26,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(blue:1,red:2,black:2),
            },
            Card {
                id: 27,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(white:1,blue:3,green:1),
            },
            Card {
                id: 28,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(white:2,blue:1),
            },
            Card {
                id: 29,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(blue:2,red:2),
            },
            Card {
                id: 30,
                tier: 1,
                color: Color::Green,
                points: 0,
                cost: macros::cost!(red:3),
            },
            Card {
                id: 31,
                tier: 1,
                color: Color::Green,
                points: 1,
                cost: macros::cost!(black:4),
            },
            Card {
                id: 32,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:1,blue:1,green:1,black:1),
            },
            Card {
                id: 33,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:2,blue:1,green:1,black:1),
            },
            Card {
                id: 34,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:2,green:1,black:2),
            },
            Card {
                id: 35,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:1,red:1,black:3),
            },
            Card {
                id: 36,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(blue:2,green:1),
            },
            Card {
                id: 37,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:2,red:2),
            },
            Card {
                id: 38,
                tier: 1,
                color: Color::Red,
                points: 0,
                cost: macros::cost!(white:3),
            },
            Card {
                id: 39,
                tier: 1,
                color: Color::Red,
                points: 1,
                cost: macros::cost!(white:4),
            },
            Card {
                id: 40,
                tier: 2,
                color: Color::Black,
                points: 1,
                cost: macros::cost!(white:3,blue:2,green:2),
            },
            Card {
                id: 41,
                tier: 2,
                color: Color::Black,
                points: 1,
                cost: macros::cost!(white:3,green:3,black:2),
            },
            Card {
                id: 42,
                tier: 2,
                color: Color::Black,
                points: 2,
                cost: macros::cost!(blue:1,green:4,red:2),
            },
            Card {
                id: 43,
                tier: 2,
                color: Color::Black,
                points: 2,
                cost: macros::cost!(green:5,red:3),
            },
            Card {
                id: 44,
                tier: 2,
                color: Color::Black,
                points: 2,
                cost: macros::cost!(white:5),
            },
            Card {
                id: 45,
                tier: 2,
                color: Color::Black,
                points: 3,
                cost: macros::cost!(black:6),
            },
            Card {
                id: 46,
                tier: 2,
                color: Color::Blue,
                points: 1,
                cost: macros::cost!(blue:2,green:2,red:3),
            },
            Card {
                id: 47,
                tier: 2,
                color: Color::Blue,
                points: 1,
                cost: macros::cost!(blue:2,green:3,black:3),
            },
            Card {
                id: 48,
                tier: 2,
                color: Color::Blue,
                points: 2,
                cost: macros::cost!(white:5,blue:3),
            },
            Card {
                id: 49,
                tier: 2,
                color: Color::Blue,
                points: 2,
                cost: macros::cost!(white:2,red:1,black:4),
            },
            Card {
                id: 50,
                tier: 2,
                color: Color::Blue,
                points: 2,
                cost: macros::cost!(blue:5),
            },
            Card {
                id: 51,
                tier: 2,
                color: Color::Blue,
                points: 3,
                cost: macros::cost!(blue:6),
            },
            Card {
                id: 52,
                tier: 2,
                color: Color::White,
                points: 1,
                cost: macros::cost!(green:3,red:2,black:2),
            },
            Card {
                id: 53,
                tier: 2,
                color: Color::White,
                points: 1,
                cost: macros::cost!(white:2,blue:3,red:3),
            },
            Card {
                id: 54,
                tier: 2,
                color: Color::White,
                points: 2,
                cost: macros::cost!(green:1,red:4,black:2),
            },
            Card {
                id: 55,
                tier: 2,
                color: Color::White,
                points: 2,
                cost: macros::cost!(red:5,black:3),
            },
            Card {
                id: 56,
                tier: 2,
                color: Color::White,
                points: 2,
                cost: macros::cost!(red:5),
            },
            Card {
                id: 57,
                tier: 2,
                color: Color::White,
                points: 3,
                cost: macros::cost!(white:6),
            },
            Card {
                id: 58,
                tier: 2,
                color: Color::Green,
                points: 1,
                cost: macros::cost!(white:3,green:2,red:3),
            },
            Card {
                id: 59,
                tier: 2,
                color: Color::Green,
                points: 1,
                cost: macros::cost!(white:2,blue:3,black:2),
            },
            Card {
                id: 60,
                tier: 2,
                color: Color::Green,
                points: 2,
                cost: macros::cost!(white:4,blue:2,black:1),
            },
            Card {
                id: 61,
                tier: 2,
                color: Color::Green,
                points: 2,
                cost: macros::cost!(blue:5,green:3),
            },
            Card {
                id: 62,
                tier: 2,
                color: Color::Green,
                points: 2,
                cost: macros::cost!(green:5),
            },
            Card {
                id: 63,
                tier: 2,
                color: Color::Green,
                points: 3,
                cost: macros::cost!(green:6),
            },
            Card {
                id: 64,
                tier: 2,
                color: Color::Red,
                points: 1,
                cost: macros::cost!(white:2,red:2,black:3),
            },
            Card {
                id: 65,
                tier: 2,
                color: Color::Red,
                points: 1,
                cost: macros::cost!(blue:3,red:2,black:3),
            },
            Card {
                id: 66,
                tier: 2,
                color: Color::Red,
                points: 2,
                cost: macros::cost!(white:1,blue:4,green:2),
            },
            Card {
                id: 67,
                tier: 2,
                color: Color::Red,
                points: 2,
                cost: macros::cost!(white:3,black:5),
            },
            Card {
                id: 68,
                tier: 2,
                color: Color::Red,
                points: 2,
                cost: macros::cost!(black:5),
            },
            Card {
                id: 69,
                tier: 2,
                color: Color::Red,
                points: 3,
                cost: macros::cost!(red:6),
            },
            Card {
                id: 70,
                tier: 3,
                color: Color::Black,
                points: 3,
                cost: macros::cost!(white:3,blue:3,green:5,red:3),
            },
            Card {
                id: 71,
                tier: 3,
                color: Color::Black,
                points: 4,
                cost: macros::cost!(red:7),
            },
            Card {
                id: 72,
                tier: 3,
                color: Color::Black,
                points: 4,
                cost: macros::cost!(green:3,red:6,black:3),
            },
            Card {
                id: 73,
                tier: 3,
                color: Color::Black,
                points: 5,
                cost: macros::cost!(red:7,black:3),
            },
            Card {
                id: 74,
                tier: 3,
                color: Color::Blue,
                points: 3,
                cost: macros::cost!(white:3,green:3,red:3,black:5),
            },
            Card {
                id: 75,
                tier: 3,
                color: Color::Blue,
                points: 4,
                cost: macros::cost!(white:7),
            },
            Card {
                id: 76,
                tier: 3,
                color: Color::Blue,
                points: 4,
                cost: macros::cost!(white:6,blue:3,black:3),
            },
            Card {
                id: 77,
                tier: 3,
                color: Color::Blue,
                points: 5,
                cost: macros::cost!(white:7,blue:3),
            },
            Card {
                id: 78,
                tier: 3,
                color: Color::White,
                points: 3,
                cost: macros::cost!(blue:3,green:3,red:5,black:3),
            },
            Card {
                id: 79,
                tier: 3,
                color: Color::White,
                points: 4,
                cost: macros::cost!(black:7),
            },
            Card {
                id: 80,
                tier: 3,
                color: Color::White,
                points: 4,
                cost: macros::cost!(white:3,red:3,black:6),
            },
            Card {
                id: 81,
                tier: 3,
                color: Color::White,
                points: 5,
                cost: macros::cost!(white:3,black:7),
            },
            Card {
                id: 82,
                tier: 3,
                color: Color::Green,
                points: 3,
                cost: macros::cost!(white:5,blue:3,red:3,black:3),
            },
            Card {
                id: 83,
                tier: 3,
                color: Color::Green,
                points: 4,
                cost: macros::cost!(blue:7),
            },
            Card {
                id: 84,
                tier: 3,
                color: Color::Green,
                points: 4,
                cost: macros::cost!(white:3,blue:6,green:3),
            },
            Card {
                id: 85,
                tier: 3,
                color: Color::Green,
                points: 5,
                cost: macros::cost!(blue:7,green:3),
            },
            Card {
                id: 86,
                tier: 3,
                color: Color::Red,
                points: 3,
                cost: macros::cost!(white:3,blue:5,green:3,black:3),
            },
            Card {
                id: 87,
                tier: 3,
                color: Color::Red,
                points: 4,
                cost: macros::cost!(green:7),
            },
            Card {
                id: 88,
                tier: 3,
                color: Color::Red,
                points: 4,
                cost: macros::cost!(blue:3,green:6,red:3),
            },
            Card {
                id: 89,
                tier: 3,
                color: Color::Red,
                points: 5,
                cost: macros::cost!(green:7,red:3),
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
