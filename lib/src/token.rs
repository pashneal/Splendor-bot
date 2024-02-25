use crate::color::Color;
use serde::{Deserialize, Serialize};
use std::cmp::{max, min};
use std::collections::HashSet;
use std::ops::{Add, AddAssign, Index, IndexMut, Sub, SubAssign};

#[derive(PartialEq, Eq, Copy, Clone, Debug, Hash, Serialize, Deserialize)]
pub struct Tokens {
    pub onyx: i8,
    pub sapphire: i8,
    pub emerald: i8,
    pub ruby: i8,
    pub diamond: i8,
    pub gold: i8,
}

impl Tokens {
    pub fn to_set(&self) -> HashSet<Color> {
        let mut set = HashSet::new();
        if self.onyx > 0 {
            set.insert(Color::Onyx);
        }
        if self.sapphire > 0 {
            set.insert(Color::Sapphire);
        }
        if self.emerald > 0 {
            set.insert(Color::Emerald);
        }
        if self.ruby > 0 {
            set.insert(Color::Ruby);
        }
        if self.diamond > 0 {
            set.insert(Color::Diamond);
        }
        set
    }

    pub fn from_vec(vec: &Vec<Color>) -> Tokens {
        let mut tokens = Tokens::empty();
        for &color in vec {
            tokens[color] += 1;
        }
        tokens
    }

    pub fn from_set(set: &HashSet<Color>) -> Tokens {
        let mut tokens = Tokens::empty();
        for color in set {
            tokens[*color] += 1;
        }
        tokens
    }

    pub fn total(&self) -> u32 {
        debug_assert!(self.legal(), "Illegal token state: {:?}", self);
        self.onyx as u32
            + self.sapphire as u32
            + self.emerald as u32
            + self.ruby as u32
            + self.diamond as u32
            + self.gold as u32
    }
    pub fn legal(&self) -> bool {
        self.onyx >= 0
            && self.sapphire >= 0
            && self.emerald >= 0
            && self.ruby >= 0
            && self.diamond >= 0
            && self.gold >= 0
    }

    pub fn empty() -> Tokens {
        Tokens {
            onyx: 0,
            sapphire: 0,
            emerald: 0,
            ruby: 0,
            diamond: 0,
            gold: 0,
        }
    }

    pub fn start(players: u8) -> Tokens {
        match players {
            2 => Tokens {
                onyx: 4,
                sapphire: 4,
                emerald: 4,
                ruby: 4,
                diamond: 4,
                gold: 5,
            },
            3 => Tokens {
                onyx: 5,
                sapphire: 5,
                emerald: 5,
                ruby: 5,
                diamond: 5,
                gold: 5,
            },
            4 => Tokens {
                onyx: 7,
                sapphire: 7,
                emerald: 7,
                ruby: 7,
                diamond: 7,
                gold: 5,
            },
            _ => panic!("Invalid number of players"),
        }
    }

    pub fn max(&self, other: &Tokens) -> Tokens {
        Tokens {
            onyx: max(self.onyx, other.onyx),
            sapphire: max(self.sapphire, other.sapphire),
            emerald: max(self.emerald, other.emerald),
            ruby: max(self.ruby, other.ruby),
            diamond: max(self.diamond, other.diamond),
            gold: max(self.gold, other.gold),
        }
    }

    pub fn one(color: Color) -> Tokens {
        let mut tokens = Tokens::empty();
        tokens[color] = 1;
        tokens
    }

    pub fn distinct(&self) -> usize {
        let mut count = 0;
        if self.onyx > 0 {
            count += 1;
        }
        if self.sapphire > 0 {
            count += 1
        }
        if self.emerald > 0 {
            count += 1
        }
        if self.ruby > 0 {
            count += 1
        }
        if self.diamond > 0 {
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
            Color::Onyx => &self.onyx,
            Color::Sapphire => &self.sapphire,
            Color::Emerald => &self.emerald,
            Color::Ruby => &self.ruby,
            Color::Diamond => &self.diamond,
            Color::Gold => &self.gold,
        }
    }
}

impl IndexMut<Color> for Tokens {
    fn index_mut<'a>(&'a mut self, color: Color) -> &'a mut i8 {
        match color {
            Color::Onyx => &mut self.onyx,
            Color::Sapphire => &mut self.sapphire,
            Color::Emerald => &mut self.emerald,
            Color::Ruby => &mut self.ruby,
            Color::Diamond => &mut self.diamond,
            Color::Gold => &mut self.gold,
        }
    }
}

impl AddAssign for Tokens {
    fn add_assign(&mut self, other: Tokens) {
        self.onyx += other.onyx;
        self.sapphire += other.sapphire;
        self.emerald += other.emerald;
        self.ruby += other.ruby;
        self.diamond += other.diamond;
        self.gold += other.gold;
        debug_assert!(self.legal());
    }
}

impl SubAssign for Tokens {
    fn sub_assign(&mut self, other: Tokens) {
        self.onyx -= other.onyx;
        self.sapphire -= other.sapphire;
        self.emerald -= other.emerald;
        self.ruby -= other.ruby;
        self.diamond -= other.diamond;
        self.gold -= other.gold;
        debug_assert!(self.legal());
    }
}

impl Add for Tokens {
    type Output = Tokens;

    fn add(self, other: Tokens) -> Tokens {
        let tokens = Tokens {
            onyx: self.onyx + other.onyx,
            sapphire: self.sapphire + other.sapphire,
            emerald: self.emerald + other.emerald,
            ruby: self.ruby + other.ruby,
            diamond: self.diamond + other.diamond,
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
            onyx: self.onyx - other.onyx,
            sapphire: self.sapphire - other.sapphire,
            emerald: self.emerald - other.emerald,
            ruby: self.ruby - other.ruby,
            diamond: self.diamond - other.diamond,
            gold: self.gold - other.gold,
        };
        debug_assert!(self.legal());
        tokens
    }
}
