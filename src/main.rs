use crate::card::dungeon::DungeonCard;

pub mod card;

#[derive(Debug)]
pub struct GameState {
    pub inventory: Vec<Box<dyn card::Ownable>>,
    pub poison: u8,
    pub health: u8,
}

impl GameState {
    pub fn increase_poison(&mut self, n: u8) {
        self.poison += n;
    }
}

fn main() {
    let mut state = GameState {
        inventory: vec![],
        poison: 0,
        health: 10,
    };
    state.inventory.push(Box::new(card::trappings::WoodenStaff{}));
    println!("Do I have a wooden staff? {}", state.inventory.first().unwrap().type_() == card::CardType::Trappings(card::Trappings::WoodenStaff));
    let card = card::dungeon::draw();
    println!("Received card {:?}", card);
    card.resolve(&mut state, true);
    println!("New state: {:?}", state);
}
