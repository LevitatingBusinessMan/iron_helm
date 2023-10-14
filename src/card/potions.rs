use super::*;
use crate::card;

card!(Health, Potions, "health.jpg" (1;1;AS) consumable);
impl Consumable for Health {
    fn consume(&self, state: &mut crate::GameState) {
        state.increase_health(4);
    }
}

#[derive(PartialEq, Debug)]
pub enum Potions {
    Health
}
