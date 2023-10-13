use crate::GameState;

use super::*;

#[derive(Debug)]
pub struct Sortab;
impl Card for Sortab {
    fn type_(&self) -> CardIdent {
        CardIdent::Characters(Characters::Sortab)
    }
    fn front(&self) -> &'static str {
        "alchemy.jpg"
    }
}

impl CharacterCard for Sortab {
    fn trappings(&self) -> Vec<&'static dyn Ownable> {
        vec![&trappings::WoodenStaff{}, &trappings::DoorWedge{}]
    }
    fn skills(&self) -> Vec<&'static dyn skills::Skill> {
        vec![&skills::Alchemy{}]
    }
}

#[derive(PartialEq, Debug)]
pub enum Characters {
    Sortab
}

pub trait CharacterCard: Card {
    fn trappings(&self) -> Vec<&'static dyn Ownable>;
    fn skills(&self) -> Vec<&'static dyn skills::Skill>;
    fn apply(&'static self, state: &mut GameState) {
        state.inventory.append(&mut self.trappings());
        state.skills.append(&mut self.skills());
    }
}
