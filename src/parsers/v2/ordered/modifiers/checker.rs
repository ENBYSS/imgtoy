use serde_yaml::Value;

use crate::parsers::v2::structure::value::{Vf64, Vusize};

#[derive(Debug)]
pub enum CheckerKind {
    Iter(Vusize),
    From {
        source: CheckerSource,
        factor: CheckerFactor,
        modulo: Option<Vusize>,
    },
}

#[derive(Debug)]
pub enum CheckerSourceKind {
    Center,
    Fixed((Vusize, Vusize)),
}

#[derive(Debug)]
pub struct CheckerSource {
    kind: CheckerSourceKind,
}

#[derive(Debug)]
pub enum CheckerFactorKind {
    Linear,
    Exponential,
}

#[derive(Debug)]
pub struct CheckerFactor {
    kind: CheckerFactorKind,
}

#[derive(Debug)]
pub struct Checker {
    chance: Vf64,
    kind: CheckerKind,
}

impl Checker {
    pub fn from_value(value: &Value) -> Option<Self> {
        None
    }
}
