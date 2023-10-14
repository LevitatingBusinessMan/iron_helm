use super::*;
use crate::card;

card!(Alchemy, Skills, "alchemy.jpg" skill);

/// Skill marker trait
pub trait Skill: Card {}

#[derive(PartialEq, Debug)]
pub enum Skills {
    Alchemy
}

