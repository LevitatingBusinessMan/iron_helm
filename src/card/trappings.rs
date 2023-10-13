use super::*;

#[derive(Debug)]
pub struct WoodenStaff;
impl Card for WoodenStaff {
    fn type_(&self) -> CardIdent {
        CardIdent::Trappings(Trappings::WoodenStaff)
    }
    fn front(&self) -> &'static str {
        "wooden_staff.jpg"
    }
}
impl Ownable for WoodenStaff {
    fn price(&'static self) -> u8 {3}
    fn weight(&'static self) -> u8 {1}
    fn location(&'static self) -> EquipLocation {EquipLocation::TwoHanded}
}

#[derive(Debug)]
pub struct DoorWedge;
impl Card for DoorWedge {
    fn type_(&self) -> CardIdent {
        CardIdent::Trappings(Trappings::DoorWedge)
    }
    fn front(&self) -> &'static str {
        "door_wedge.jpg"
    }
}
impl Ownable for DoorWedge {
    fn price(&'static self) -> u8 {1}
    fn weight(&'static self) -> u8 {1}
    fn location(&'static self) -> EquipLocation {EquipLocation::Accessory}
}

#[derive(PartialEq, Debug)]
pub enum Trappings {
    WoodenStaff,
    DoorWedge,
}
