use super::*;
use crate::card;

card!(Altar, Dungeon, "altar.jpg");
impl DungeonCard for Altar {
    fn resolve(&self, state: &mut crate::GameState, _first_draw: bool) {
        let choice = crate::choice::select(
            "Sometimes a sanctuary is found in the least expected place. You drop and pray at the altar for?",
            &vec!["A blessing", "Healing"]
        );
        match choice {
            "A blessing" => {
                state.increase_blessings(1);
            },
            "Healing" => {
                use crate::dice::d6;
                state.decrease_poison(d6()-d6());
            },
            _ => unreachable!()
        }
    }
}
card!(Ambush, Dungeon, "ambush.jpg");
impl DungeonCard for Ambush {}
card!(ArrowTrap, Dungeon, "arrow_trap.jpg");
impl DungeonCard for ArrowTrap {
    fn resolve(&self, state: &mut crate::GameState, first_draw: bool) {
        state.increase_poison(if first_draw {1} else {2});
    }
}
card!(Campsite, Dungeon, "campsite.jpg");
impl DungeonCard for Campsite {}
card!(Clearing, Dungeon, "clearing.jpg");
impl DungeonCard for Clearing {}
card!(Labyrinth, Dungeon, "labyrinth.jpg");
impl DungeonCard for Labyrinth {}
card!(Merchant, Dungeon, "merchant.jpg");
impl DungeonCard for Merchant {}
card!(MushroomGrove, Dungeon, "mushroom_grove.jpg");
impl DungeonCard for MushroomGrove {}
card!(Skirmish, Dungeon, "skirmish.jpg");
impl DungeonCard for Skirmish {}
card!(Treasure, Dungeon, "treasure.jpg");
impl DungeonCard for Treasure {}


#[derive(PartialEq, Debug)]
pub enum Dungeon {
    Altar,
    Ambush, // 3 times
    ArrowTrap,
    Campsite,
    Clearing,
    Labyrinth,
    Merchant,
    MushroomGrove,
    Skirmish, // 4 times
    Treasure,
}

/// A dungeon card that has to be resolved
pub trait DungeonCard: Card {
    fn resolve(&self, state: &mut crate::GameState, first_draw: bool) {
        unimplemented!()
    }
}

pub fn draw() -> &'static dyn DungeonCard {
    // 15 possible options
    let r = rand::random::<u8>() % 15;
    match r {
        0 => &Altar{},
        1..=3 => &Ambush{},
        4 => &ArrowTrap{},
        5 => &Campsite{},
        6 => &Clearing{},
        7 => &Labyrinth{},
        8 => &Merchant{},
        9 => &MushroomGrove{},
        10..=13 => &Skirmish{},
        14 => &Treasure{},
        _ => unreachable!()
    }
}
