use crate::parsers::v2::structure::{Vf64, Vusize};

pub enum CheckerKind {}

pub enum CheckerSourceKind {}

pub struct CheckerSource {
    kind: CheckerSourceKind,
}

pub enum CheckerFactorKind {}

pub struct CheckerFactor {
    kind: CheckerFactorKind,
}

pub struct Checker {
    chance: Vf64,
    kind: CheckerKind,
    source: CheckerSource,
    factor: CheckerFactor,
    modulo: Vusize,
}
