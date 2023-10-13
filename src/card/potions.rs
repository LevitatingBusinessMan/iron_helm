use super::*;

#[derive(Debug)]
pub struct Health;
impl Card for Health {
    fn type_(&self) -> CardIdent {
        CardIdent::Potions(Potions::Health)
    }
    fn front(&self) -> &'static str {
        "health.jpg"
    }
    fn consumable(&'static self) -> Option<&'static dyn Consumable> {Some(self)}
}

impl Ownable for Health {
    fn price(&'static self) -> u8 {1}
    fn weight(&'static self) -> u8 {1}
    fn location(&'static self) -> EquipLocation {EquipLocation::Accessory}
}

impl Consumable for Health {
    fn consume(&self, state: &mut crate::GameState) {
        state.increase_health(4);
    }
}

#[derive(PartialEq, Debug)]
pub enum Potions {
    Health
}
