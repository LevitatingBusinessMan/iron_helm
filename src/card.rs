pub mod trappings;
pub use trappings::Trappings;
pub mod dungeon;
pub use dungeon::Dungeon;

#[derive(PartialEq, Debug)]
/// Basically an index of all cards
pub enum CardType {
    Trappings(Trappings),
    Dungeon(Dungeon),
}

/// Generic Card trait
pub trait Card: std::fmt::Debug {
    fn type_(&self) -> CardType;
    fn front(&self) -> &'static str;
    fn back(&self) -> &'static str {
        match self.type_() {
            CardType::Trappings(_) => "trappings.jpg",
            CardType::Dungeon(_) => "dungeon.jpg",
        }
    }
}

/// Marker trait for ownable cards
pub trait Ownable: Card {}

/// Marker trait for consumables
pub trait Consumable: Ownable {
    fn consume(state: &mut crate::GameState) -> () {

    }
}
