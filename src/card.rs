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

use self::dungeon::DungeonCard;

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
}

/// Generic Card trait
// https://stackoverflow.com/a/30275713/8935250
// https://github.com/rust-lang/rust/issues/31844
pub trait Card: std::fmt::Debug {
    fn type_(&self) -> CardIdent;
    fn front(&self) -> &'static str;
    fn back(&self) -> &'static str {
        match self.type_() {
            CardIdent::Trappings(_) => "trappings.jpg",
            CardIdent::Dungeon(_) => "dungeon.jpg",
            CardIdent::Skills(_) => "skills.jpg",
            CardIdent::Characters(_) => "characters.jpg",
            CardIdent::Loot(_) => "loot.jpg",
            CardIdent::Potions(_) => "potions.jpg",
        }
    }
    fn ownable(&'static self) -> Option<&'static dyn Ownable> {None}
    fn consumable(&'static self) -> Option<&'static dyn Consumable> {None}
    fn dungeon_card(&'static self) -> Option<&'static dyn DungeonCard> {None}
}

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
}

#[macro_export]
/// Macros for creating cards
macro_rules! card {
    ($c:ident, $t:ident, $f:literal ($p:expr;$w:expr;$l:ident) consumable) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&self) -> &'static str {
                $f
            }
            fn ownable(&'static self) -> Option<&'static dyn Ownable> {Some(self)}
            fn consumable(&'static self) -> Option<&'static dyn Consumable> {Some(self)}
        }
        impl Ownable for $c {
            fn price(&'static self) -> u8 {$p}
            fn weight(&'static self) -> u8 {$w}
            fn location(&'static self) -> EquipLocation {EquipLocation::$l}
        }
    };
    ($c:ident, $t:ident, $f:literal ($p:expr;$w:expr;$l:ident)) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&self) -> &'static str {
                $f
            }
            fn ownable(&'static self) -> Option<&'static dyn Ownable> {Some(self)}
        }
        impl Ownable for $c {
            fn price(&'static self) -> u8 {$p}
            fn weight(&'static self) -> u8 {$w}
            fn location(&'static self) -> EquipLocation {EquipLocation::$l}
        }
    };
    ($c:ident, $t:ident, $f:literal dungeon) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&self) -> &'static str {
                $f
            }
            fn dungeon_card(&'static self) -> Option<&'static dyn DungeonCard> {Some(self)}
        }
    };
    ($c:ident, $t:ident, $f:literal skill) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&self) -> &'static str {
                $f
            }
        }
        impl Skill for $c {}
    };
    ($c:ident, $t:ident, $f:literal) => {
        #[derive(Debug)]
        pub struct $c;
        impl Card for $c {
            fn type_(&self) -> CardIdent {
                CardIdent::$t($t::$c)
            }
            fn front(&self) -> &'static str {
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

/// Marker trait for consumables
pub trait Consumable: Ownable {
    fn consume(&'static self, state: &mut crate::GameState);
}
