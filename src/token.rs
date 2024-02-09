use crate::color::Color;
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash)]
pub struct Tokens {
    pub black: i8,
    pub blue: i8,
    pub green: i8,
    pub red: i8,
    pub white: i8,
    pub gold: i8,
}

impl Tokens {
    pub fn from_set(set: &HashSet<Color>) -> Tokens {
        let mut tokens = Tokens::empty();
        for color in set {
            tokens[*color] += 1;
        }
        tokens
    }

    pub fn total(&self) -> u32 {
        debug_assert!(self.legal(), "Illegal token state: {:?}", self);
        self.black as u32
            + self.blue as u32
            + self.green as u32
            + self.red as u32
            + self.white as u32
            + self.gold as u32
    }
    pub fn legal(&self) -> bool {
        self.black >= 0
            && self.blue >= 0
            && self.green >= 0
            && self.red >= 0
            && self.white >= 0
            && self.gold >= 0
    }

    pub fn empty() -> Tokens {
        Tokens {
            black: 0,
            blue: 0,
            green: 0,
            red: 0,
            white: 0,
            gold: 0,
        }
    }

    pub fn start(players: u8) -> Tokens {
        match players {
            2 => Tokens {
                black: 4,
                blue: 4,
                green: 4,
                red: 4,
                white: 4,
                gold: 5,
            },
            3 => Tokens {
                black: 5,
                blue: 5,
                green: 5,
                red: 5,
                white: 5,
                gold: 5,
            },
            4 => Tokens {
                black: 7,
                blue: 7,
                green: 7,
                red: 7,
                white: 7,
                gold: 5,
            },
            _ => panic!("Invalid number of players"),
        }
    }

    pub fn max(&self, other: &Tokens) -> Tokens {
        Tokens {
            black: max(self.black, other.black),
            blue: max(self.blue, other.blue),
            green: max(self.green, other.green),
            red: max(self.red, other.red),
            white: max(self.white, other.white),
            gold: max(self.gold, other.gold),
        }
    }

    pub fn one(color: Color) -> Tokens {
        let mut tokens = Tokens::empty();
        tokens[color] = 1;
        tokens
    }

    pub fn piles(&self) -> usize {
        let mut count = 0;
        if self.black > 0 {
            count += 1;
        }
        if self.blue > 0 {
            count += 1
        }
        if self.green > 0 {
            count += 1
        }
        if self.red > 0 {
            count += 1
        }
        if self.white > 0 {
            count += 1
        }
        count
    }
    pub fn can_buy(&self, other: &Tokens) -> bool {
        unimplemented!()
    }
}

impl Index<Color> for Tokens {
    type Output = i8;

    fn index<'a>(&'a self, color: Color) -> &'a i8 {
        match color {
            Color::Black => &self.black,
            Color::Blue => &self.blue,
            Color::Green => &self.green,
            Color::Red => &self.red,
            Color::White => &self.white,
            Color::Gold => &self.gold,
        }
    }
}

impl IndexMut<Color> for Tokens {
    fn index_mut<'a>(&'a mut self, color: Color) -> &'a mut i8 {
        match color {
            Color::Black => &mut self.black,
            Color::Blue => &mut self.blue,
            Color::Green => &mut self.green,
            Color::Red => &mut self.red,
            Color::White => &mut self.white,
            Color::Gold => &mut self.gold,
        }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Tokens) {
        self.black += other.black;
        self.blue += other.blue;
        self.green += other.green;
        self.red += other.red;
        self.white += other.white;
        self.gold += other.gold;
        debug_assert!(self.legal());
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Tokens) {
        self.black -= other.black;
        self.blue -= other.blue;
        self.green -= other.green;
        self.red -= other.red;
        self.white -= other.white;
        self.gold -= other.gold;
        debug_assert!(self.legal());
    }
}

impl Add for Tokens {
    type Output = Tokens;

    fn add(self, other: Tokens) -> Tokens {
        let tokens = Tokens {
            black: self.black + other.black,
            blue: self.blue + other.blue,
            green: self.green + other.green,
            red: self.red + other.red,
            white: self.white + other.white,
            gold: self.gold + other.gold,
        };
        debug_assert!(self.legal());
        tokens
    }
}

impl Sub for Tokens {
    type Output = Tokens;

    fn sub(self, other: Tokens) -> Tokens {
        let tokens = Tokens {
            black: self.black - other.black,
            blue: self.blue - other.blue,
            green: self.green - other.green,
            red: self.red - other.red,
            white: self.white - other.white,
            gold: self.gold - other.gold,
        };
        debug_assert!(self.legal());
        tokens
    }
}
