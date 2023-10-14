use super::*;
use crate::card;

card!(HidInn, Plots, "hidd_inn.jpg");

#[derive(PartialEq, Debug)]
pub enum Plots {
    HidInn,
}
