use crate::card;

use super::*;

card!(Ration, Loot, "ration.jpg" (1;1;AS));
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
