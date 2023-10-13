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

#[derive(PartialEq, Debug)]
/// Basically an index of all cards,
/// used for identifying a card.
pub enum CardIdent {
    Trappings(Trappings),
    Dungeon(Dungeon),
    Characters(Characters),
    Skills(Skills),
    Loot(Loot),
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
        }
    }
    fn ownable(&self) -> Option<&'static dyn Ownable> {None}
    fn consumable(&'static self) -> Option<&'static dyn Consumable> {None}
}

/// Marker trait for ownable cards
pub trait Ownable: Card {}

/// Marker trait for consumables
pub trait Consumable: Ownable {
    fn consume(&'static self, state: &mut crate::GameState);
}
