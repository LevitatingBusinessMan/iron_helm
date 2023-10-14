pub mod trappings;
pub use trappings::Trappings;
pub mod dungeon;
pub use dungeon::Dungeon;
pub mod characters;
pub use characters::Characters;
pub mod skills;
pub use skills::Skills;
pub mod loot;
pub use loot::Loot;
pub mod potions;
pub use potions::Potions;
pub mod plots;
pub use plots::Plots;

use crate::GameState;

#[derive(PartialEq, Debug)]
/// Basically an index of all cards,
/// used for identifying a card.
pub enum CardIdent {
    Trappings(Trappings),
    Dungeon(Dungeon),
    Characters(Characters),
    Skills(Skills),
    Loot(Loot),
    Potions(Potions),
    Plots(Plots),
}

/// Generic Card trait
// https://stackoverflow.com/a/30275713/8935250
// https://github.com/rust-lang/rust/issues/31844
pub trait Card: std::fmt::Debug {
    fn type_(&'static self) -> CardIdent;
    fn front(&'static self) -> &'static str;
    fn back(&'static self) -> &'static str {
        match self.type_() {
            CardIdent::Trappings(_) => "trappings.jpg",
            CardIdent::Dungeon(_) => "dungeon.jpg",
            CardIdent::Skills(_) => "skills.jpg",
            CardIdent::Characters(_) => "characters.jpg",
            CardIdent::Loot(_) => "loot.jpg",
            CardIdent::Potions(_) => "potions.jpg",
            CardIdent::Plots(_) => "plots.jpg",
        }
    }
    fn ownable(&'static self) -> Option<&'static dyn Ownable> {None}
    fn consumable(&'static self) -> Option<&'static dyn Consumable> {None}
}

#[derive(PartialEq)]
pub enum EquipLocation {
    PH,
    OH,
    TH,
    HD,
    BD,
    AS,
}

/// Marker trait for ownable cards
pub trait Ownable: Card {
    fn price(&'static self) -> u8;
    fn weight(&'static self) -> u8;
    fn location(&'static self) -> EquipLocation;
    /// For use within the trait
    /// See: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=29127510ff5c3a4a8ed16d90575e3a20
    fn as_trait(&'static self) -> &'static dyn Ownable;
    fn equip(&'static self, state: &mut GameState) -> Result<(), &'static str> {
        if let Some(i) = state.inventory.iter().position(|c| c.type_() == self.type_()) {
            match self.location() {
                EquipLocation::AS => Err("This item cannot be equipped"),
                _ => {
                    match self.location() {
                        EquipLocation::PH => {
                            if let Some(cur) = state.primary {
                                state.inventory.push(cur);
                                if cur.location() == EquipLocation::TH {
                                    state.offhand = None;
                                }
                            }
                        }
                        EquipLocation::OH => {
                            if let Some(cur) = state.offhand {
                                state.inventory.push(cur);
                                if cur.location() == EquipLocation::TH {
                                    state.primary = None;
                                }
                            }
                            state.offhand = Some(self.as_trait());
                        },
                        EquipLocation::TH => {
                            if let Some(cur_prime) = state.primary {
                                state.inventory.push(cur_prime);
                                if let Some(cur_off) = state.offhand && cur_prime.location() != EquipLocation::TH {
                                    state.inventory.push(cur_off);
                                }
                            }
                            state.primary = Some(self.as_trait());
                            state.offhand = Some(self.as_trait());
                        },
                        EquipLocation::HD => {
                            if let Some(cur) = state.head {
                                state.inventory.push(cur);
                            }
                            state.head = Some(self.as_trait());
                        },
                        EquipLocation::BD => {
                            if let Some(cur) = state.body {
                                state.inventory.push(cur);
                            }
                            state.body = Some(self.as_trait());
                        },
                        EquipLocation::AS => unreachable!(),
                    }
                    state.inventory.remove(i);
                    Ok(())
                }
            }
        } else {
            Err("This item is not in inventory")
        }
    }
}

/// Marker trait for consumables
pub trait Consumable: Ownable {
    fn consume(&'static self, state: &mut crate::GameState);
}

#[macro_export]
/// Macros for creating cards
macro_rules! card {
    ($c:ident, $t:ident, $f:literal ($p:expr;$w:expr;$l:ident) consumable) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&'static self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&'static self) -> &'static str {
                $f
            }
            fn ownable(&'static self) -> Option<&'static dyn Ownable> {Some(self)}
            fn consumable(&'static self) -> Option<&'static dyn Consumable> {Some(self)}
        }
        impl Ownable for $c {
            fn price(&'static self) -> u8 {$p}
            fn weight(&'static self) -> u8 {$w}
            fn location(&'static self) -> EquipLocation {EquipLocation::$l}
            fn as_trait(&'static self) -> &'static dyn Ownable {self}
        }
    };
    ($c:ident, $t:ident, $f:literal ($p:expr;$w:expr;$l:ident)) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&'static self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&'static self) -> &'static str {
                $f
            }
            fn ownable(&'static self) -> Option<&'static dyn Ownable> {Some(self)}
        }
        impl Ownable for $c {
            fn price(&'static self) -> u8 {$p}
            fn weight(&'static self) -> u8 {$w}
            fn location(&'static self) -> EquipLocation {EquipLocation::$l}
            fn as_trait(&'static self) -> &'static dyn Ownable {self}
        }
    };
    ($c:ident, $t:ident, $f:literal skill) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&'static self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&'static self) -> &'static str {
                $f
            }
        }
        impl Skill for $c {}
    };
    ($c:ident, $t:ident, $f:literal) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&'static self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&'static self) -> &'static str {
                $f
            }
        }
    };
}

#[macro_export]
/// Create an ownable implementation
macro_rules! ownable {
    ($p:expr;$w:expr;$l:ident for $c:ident) => {
        impl Ownable for $c {
            fn price(&'static self) -> u8 {$p}
            fn weight(&'static self) -> u8 {$w}
            fn location(&'static self) -> EquipLocation {EquipLocation::$l}
        }
    };
}
