#![feature(return_position_impl_trait_in_trait)]
#![feature(associated_type_defaults)]
#![feature(let_chains)]

#[macro_use]
pub mod card;
use std::fmt::Write;

use card::*;

pub mod dice;
pub mod choice;

#[derive(Debug)]
pub struct GameState {
    pub inventory: Vec<&'static dyn Ownable>,
    pub skills: Vec<&'static dyn skills::Skill>,
    pub poison: u8,
    pub health: u8,
    pub energy: u8,
    pub rations: u8,
    pub wealth: u8,
    pub blessings: u8,
    pub character: &'static dyn characters::CharacterCard,
    pub head: Option<&'static dyn Ownable>,
    pub body: Option<&'static dyn Ownable>,
    pub primary: Option<&'static dyn Ownable>,
    pub offhand: Option<&'static dyn Ownable>,
}

impl GameState {
    pub fn increase_poison(&mut self, n: u8) {
        println!("You've gained {n} poison tokens!");
        self.poison += n;
    }
    pub fn decrease_poison(&mut self, n: u8) {
        if self.poison < n {
            println!("You've lost {} poison tokens!", self.poison);
            self.poison = 0
        } else {
            println!("You've lost {n} poison tokens!");
            self.poison -= n;
        }
    }
    pub fn increase_energy(&mut self, n: u8) {
        println!("You've gained {n} energy tokens!");
        self.energy += n;
    }
    pub fn increase_health(&mut self, n: u8) {
        println!("You've gained {n} health tokens!");
        self.health += n;
    }
    pub fn increase_blessings(&mut self, n: u8) {
        println!("You've gained {n} blessing tokens!");
        self.blessings += n;
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
        blessings: 0,
        character: &characters::Sortab{},
        head: None,
        primary: None,
        body: None,
        offhand: None,
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
    let potion = &card::potions::Health;
    if let Some(potion) = potion.consumable() {
        potion.consume(&mut state);
        println!("Drank potion. New state: {:?}", state);
    }
}
