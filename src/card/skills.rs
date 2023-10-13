use super::*;

#[derive(Debug)]
pub struct Alchemy;
impl Card for Alchemy {
    fn type_(&self) -> CardIdent {
        CardIdent::Skills(Skills::Alchemy)
    }
    fn front(&self) -> &'static str {
        "alchemy.jpg"
    }
}
impl Skill for Alchemy {}

/// Skill marker trait
pub trait Skill: Card {}

#[derive(PartialEq, Debug)]
pub enum Skills {
    Alchemy
}

