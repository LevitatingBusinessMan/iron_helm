use super::*;

#[derive(Debug)]
pub struct Ration;
impl Card for Ration {
    fn type_(&self) -> CardIdent {
        CardIdent::Loot(Loot::Ration)
    }
    fn front(&self) -> &'static str {
        "ration.jpg"
    }
    fn consumable(&'static self) -> Option<&'static dyn Consumable> {Some(self)}
}

impl Ownable for Ration {}

impl Consumable for Ration {
    fn consume(&self, state: &mut crate::GameState) {
        state.increase_energy(2);
    }
}

#[derive(PartialEq, Debug)]
pub enum Loot {
    Ration
}

pub fn draw() -> &'static dyn Card {
    return &Ration{}
}
