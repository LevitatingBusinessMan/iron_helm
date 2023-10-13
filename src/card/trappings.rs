use super::*;

pub struct WoodenStaff;
impl Card for WoodenStaff {
    fn type_(&self) -> CardType {
        CardType::Trappings(Trappings::WoodenStaff)
    }
    fn front(&self) -> &'static str {
        "wooden_staff.jpg"
    }
}
impl Ownable for WoodenStaff {}

#[derive(PartialEq)]
pub enum Trappings {
    WoodenStaff
}
