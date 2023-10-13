pub mod trappings;
pub use trappings::Trappings;

#[derive(PartialEq)]
/// Basically an index of all cards
pub enum CardType {
    Trappings(Trappings),
}

/// Generic Card trait
pub trait Card {
    fn type_(&self) -> CardType;
    fn front(&self) -> &'static str;
}

/// Marker trait for ownable cards
pub trait Ownable: Card {}

/// Marker trait for consumables
pub trait Consumable: Ownable {
    fn consume(state: &mut crate::GameState) -> () {

    }
}
