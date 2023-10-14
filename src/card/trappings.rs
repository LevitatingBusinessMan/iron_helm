use super::*;
use crate::card;

card!(WoodenStaff, Trappings, "wooden_staff.jpg" (3;1;TH));
card!(DoorWedge, Trappings, "door_wedge.jpg" (1;1;AS));

#[derive(PartialEq, Debug)]
pub enum Trappings {
    WoodenStaff,
    DoorWedge,
}
