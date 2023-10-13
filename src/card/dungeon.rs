use super::*;

#[derive(Debug)]
pub struct ArrowTrap;
impl Card for ArrowTrap {
    fn type_(&self) -> CardType {
        CardType::Dungeon(Dungeon::ArrowTrap)
    }
    fn front(&self) -> &'static str {
        "arrow_trap.jpg"
    }
}

impl DungeonCard for ArrowTrap {
    fn resolve(&self, state: &mut crate::GameState, first_draw: bool) {
        state.increase_poison(if first_draw {1} else {2});
    }
}

#[derive(PartialEq, Debug)]
pub enum Dungeon {
    ArrowTrap
}

/// A dungeon card that has to be resolved
pub trait DungeonCard: Card {
    fn resolve(&self, state: &mut crate::GameState, first_draw: bool);
}

pub fn draw() -> impl DungeonCard {
    return ArrowTrap{};
}
