pub mod card;

pub struct GameState {
    pub inventory: Vec<Box<dyn card::Ownable>>,
    pub health: u8,
}

fn main() {
    let mut state = GameState {
        inventory: vec![],
        health: 10,
    };
    state.inventory.push(Box::new(card::trappings::WoodenStaff{}));
    println!("Do I have a wooden staff? {}", state.inventory.first().unwrap().type_() == card::CardType::Trappings(card::Trappings::WoodenStaff));
}
