use crate::card::*;
use crate::token::Tokens;

#[derive(Debug, Clone)]
pub struct Player{
    points : u8,
    reserved : Vec<CardId>,
    gems : Tokens,
    developments : Tokens,
}

impl Player{
    pub fn new() -> Player{
        Player{
            points : 0,
            reserved : Vec::new(),
            gems : Tokens::empty(),
            developments : Tokens::empty(),
        }
    }

    pub fn points(&self) -> u8{
        self.points
    }

    pub fn reserved(&self) -> &Vec<u8>{
        &self.reserved
    }
    
    pub fn gems(&self) -> &Tokens{
        &self.gems
    }


    pub fn developments(&self) -> &Tokens{
        &self.developments
    }

    pub fn add_gems(&mut self, gems : Tokens) {
        self.gems += gems;
    }

    pub fn purchase_card(&mut self, card : &Card, payment: &Tokens) {
        debug_assert!(payment.legal());
        self.gems -= *payment;
        debug_assert!(self.gems.legal());
        self.developments += Tokens::one(card.color());
        self.points += card.points();
        self.reserved.retain(|&x| x != card.id());
    }

    pub fn reserve_card(&mut self, card : Card) {
        debug_assert!(self.reserved.len() < 3);
        self.reserved.push(card.id());
    }

}
