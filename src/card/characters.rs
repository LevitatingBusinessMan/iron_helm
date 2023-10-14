use crate::{GameState, card};

use super::*;

card!(Sortab,Characters, "sortab.jpg");
impl CharacterCard for Sortab {
    fn trappings(&self) -> Vec<&'static dyn Ownable> {
        vec![&trappings::WoodenStaff{}, &trappings::DoorWedge{}]
    }
    fn skills(&self) -> Vec<&'static dyn skills::Skill> {
        vec![&skills::Alchemy{}]
    }
    fn health(&'static self) -> u8 {12}
    fn energy(&'static self) -> u8 {12}
    fn encumberance(&'static self) -> u8 {9}
    fn rations(&'static self) -> u8 {1}
    fn wealth(&'static self) -> u8 {3}
    fn proficiency(&'static self) -> Proficiency {Proficiency::Agility}
}

#[derive(PartialEq, Debug)]
pub enum Characters {
    Sortab
}

pub enum Proficiency {
    Agility,
    Brawn,
    Mind,
}

pub trait CharacterCard: Card {
    fn trappings(&'static self) -> Vec<&'static dyn Ownable>;
    fn skills(&'static self) -> Vec<&'static dyn skills::Skill>;
    fn health(&'static self) -> u8;
    fn energy(&'static self) -> u8;
    fn encumberance(&'static self) -> u8;
    fn rations(&'static self) -> u8;
    fn wealth(&'static self) -> u8;
    fn proficiency(&'static self) -> Proficiency;
    fn apply(&'static self, state: &mut GameState) {
        state.inventory.append(&mut self.trappings());
        state.skills.append(&mut self.skills());
        state.health = self.health();
        state.energy = self.energy();
        state.rations = self.rations();
        state.wealth = self.wealth();
    }
}
