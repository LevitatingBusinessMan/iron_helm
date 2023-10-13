#![feature(return_position_impl_trait_in_trait)]

pub mod card;
use card::*;

#[derive(Debug)]
pub struct GameState {
    pub inventory: Vec<&'static dyn Ownable>,
    pub skills: Vec<&'static dyn skills::Skill>,
    pub poison: u8,
    pub health: u8,
    pub energy: u8,
    pub rations: u8,
    pub wealth: u8,
    pub character: &'static dyn characters::CharacterCard,
}

impl GameState {
    pub fn increase_poison(&mut self, n: u8) {
        self.poison += n;
    }
    pub fn increase_energy(&mut self, n: u8) {
        self.energy += n;
    }
    pub fn increase_health(&mut self, n: u8) {
        self.health += n;
    }
}

fn main() {
    let mut state = GameState {
        inventory: vec![],
        skills: vec![],
        poison: 0,
        health: 0,
        energy: 0,
        rations: 0,
        wealth: 0,
        character: &characters::Sortab{},
    };
    state.character.apply(&mut state);
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
