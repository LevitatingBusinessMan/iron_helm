use super::*;
use crate::card;

card!(Axe, Trappings, "axe.jpg" (2;2;PH));
card!(Buckler, Trappings, "buckler.jpg" (2;1;OH));
card!(Circlet, Trappings, "circlet.jpg" (3;1;HD));
card!(Dagger, Trappings, "dagger.jpg" (2;1;PH));
card!(DoorWedge, Trappings, "door_wedge.jpg" (1;1;AS));
card!(LeatherArmor, Trappings, "leather_armor" (3;2;BD));
card!(Mace, Trappings, "mace.jpg" (3;2;PH));
card!(Rations, Trappings, "rations.jpg" (1;0;AS));
card!(Robes, Trappings, "robes.jpg" (2;1;BD));
card!(Shield, Trappings, "shield.jpg" (4;3;OH));
card!(ShortBow, Trappings, "short_bow.jpg" (3;1;TH));
card!(Torch, Trappings, "torch.jpg" (1;1;AS));
card!(WoodenStaff, Trappings, "wooden_staff.jpg" (3;1;TH));

#[derive(PartialEq, Debug)]
pub enum Trappings {
    Axe,
    Buckler,
    Circlet,
    Dagger,
    DoorWedge,
    LeatherArmor,
    Mace,
    Rations,
    Robes,
    Shield,
    ShortBow,
    Torch,
    WoodenStaff
}
