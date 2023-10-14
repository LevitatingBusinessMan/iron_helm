use super::*;
use crate::card;

card!(ArrowTrap, Dungeon, "arrow_trap.jpg" dungeon);
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

pub fn draw() -> &'static dyn DungeonCard {
    return &ArrowTrap{};
}
