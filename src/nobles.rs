use crate::token::Tokens;

pub type NobleId = u8;

#[derive(Debug, Clone)]
pub struct Noble {
    points : u8,
    id : NobleId,
    requirements : Tokens,
}


impl Noble {
    pub fn all() -> Vec<Noble> {
        vec![
            Noble::new(3, 0, Tokens { 
                black: 0, 
                blue: 0,
                green: 4,
                red: 4,
                white: 0,
                gold: 0 }
            ),
            Noble::new(3, 1, Tokens { 
                black: 3, 
                blue: 0,
                green: 0,
                red: 3,
                white: 3,
                gold: 0 }
            ),
            Noble::new(3, 2, Tokens { 
                black: 3, 
                blue: 0,
                green: 3,
                red: 3,
                white: 0,
                gold: 0 }
            ),
            Noble::new(3, 3, Tokens { 
                black: 0, 
                blue: 4,
                green: 0,
                red: 0,
                white: 4,
                gold: 0 }
            ),
            Noble::new(3, 4, Tokens { 
                black: 4, 
                blue: 0,
                green: 0,
                red: 0,
                white: 4,
                gold: 0 }
            ),
            Noble::new(3, 5, Tokens { 
                black: 0, 
                blue: 4,
                green: 4,
                red: 0,
                white: 0,
                gold: 0 }
            ),
            Noble::new(3, 6, Tokens { 
                black: 0, 
                blue: 3,
                green: 3,
                red: 3,
                white: 0,
                gold: 0 }
            ),
            Noble::new(3, 7, Tokens { 
                black: 0, 
                blue: 3,
                green: 3,
                red: 0,
                white: 3,
                gold: 0 }
            ),
            Noble::new(3, 8, Tokens { 
                black: 4, 
                blue: 0,
                green: 4,
                red: 0,
                white: 0,
                gold: 0 }
            ),
            Noble::new(3, 9, Tokens { 
                black: 3, 
                blue: 3,
                green: 0,
                red: 0,
                white: 3,
                gold: 0 }
            ),
        ]
    }
    fn new(points: u8, id: NobleId, requirements: Tokens) -> Noble {
        Noble {
            points,
            id,
            requirements,
        }
    }
    pub fn is_attracted(&self , developments: &Tokens) -> bool {
        let can_attract =
            (developments.black  > self.requirements.black) &&
            (developments.blue  > self.requirements.blue) &&
            (developments.green > self.requirements.green) &&
            (developments.red   > self.requirements.red) &&
            (developments.white > self.requirements.white);

        can_attract
    }
}
