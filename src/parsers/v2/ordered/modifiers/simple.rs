use crate::parsers::v2::structure::{Vf64, Vusize};

pub struct Invert {
    chance: Vf64,
}

pub struct Exponentiate {
    chance: Vf64,
    factor: Vf64,
}

pub struct Blur {
    chance: Vf64,
    factor: Vusize,
}
