use serde_yaml::Value;

use crate::parsers::v2::structure::value::{Vf64, Vusize};

pub enum CheckerKind {
    Iter(Vusize),
    From {
        source: CheckerSource,
        factor: CheckerFactor,
        modulo: Option<Vusize>,
    },
}

pub enum CheckerSourceKind {
    Center,
    Fixed((Vusize, Vusize)),
}

pub struct CheckerSource {
    kind: CheckerSourceKind,
}

pub enum CheckerFactorKind {
    Linear,
    Exponential,
}

pub struct CheckerFactor {
    kind: CheckerFactorKind,
}

pub struct Checker {
    chance: Vf64,
    kind: CheckerKind,
}

impl Checker {
    pub fn from_value(value: &Value) -> Option<Self> {
        None
    }
}
