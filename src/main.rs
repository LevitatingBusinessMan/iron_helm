#![feature(return_position_impl_trait_in_trait)]
use card::Ownable;

pub mod card;

#[derive(Debug)]
pub struct GameState {
    pub inventory: Vec<&'static dyn Ownable>,
    pub poison: u8,
    pub health: u8,
    pub energy: u8,
}

impl GameState {
    pub fn increase_poison(&mut self, n: u8) {
        self.poison += n;
    }
    pub fn increase_energy(&mut self, n: u8) {
        self.energy += n;
    }
}

fn main() {
    let mut state = GameState {
        inventory: vec![],
        poison: 0,
        health: 10,
        energy: 10,
    };
    state.inventory.push(&card::trappings::WoodenStaff{});
    println!("Do I have a wooden staff? {}", state.inventory.first().unwrap().type_() == card::CardIdent::Trappings(card::Trappings::WoodenStaff));
    let card = card::dungeon::draw();
    println!("Received card {:?}", card);
    card.resolve(&mut state, true);
    println!("New state: {:?}", state);
    let loot = card::loot::draw();
    println!("Received loot: {:?}", loot);
    if let Some(loot) = loot.consumable() {
        println!("Loot is consumable, consuming");
        loot.consume(&mut state);
        println!("New state: {:?}", state);
    }
}
