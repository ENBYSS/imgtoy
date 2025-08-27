use image_effects::dither::ordered::tools::properties;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, Chance, Vf64, Vusize,
};

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
    Fixed { x: Vusize, y: Vusize },
}

#[derive(Debug)]
pub struct CheckerSource {
    kind: CheckerSourceKind,
}

#[derive(Debug)]
pub enum CheckerFactorKind {
    Linear,
    Exponential { factor: Vf64 },
}

#[derive(Debug)]
pub struct CheckerFactor {
    kind: CheckerFactorKind,
}

#[derive(Debug)]
pub struct Checker {
    chance: Chance,
    kind: CheckerKind,
}

impl Checker {
    fn parse_source(value: &Value) -> CheckerSource {
        let source = value.get("source").unwrap();

        let kind = source.get("type").unwrap().as_str().unwrap();

        CheckerSource {
            kind: match kind {
                "center" => CheckerSourceKind::Center,
                "fixed" => {
                    let fixed = source.get("fixed").unwrap();
                    let x = parse_property_as_usize(fixed, "x").unwrap();
                    let y = parse_property_as_usize(fixed, "y").unwrap();

                    CheckerSourceKind::Fixed { y, x }
                }
                _ => panic!("something's wrong with the source"),
            },
        }
    }

    fn parse_factor(value: &Value) -> CheckerFactor {
        let factor = value.get("factor").unwrap();

        let kind = factor.get("type").unwrap().as_str().unwrap();

        CheckerFactor {
            kind: match kind {
                "linear" => CheckerFactorKind::Linear,
                "exponential" => CheckerFactorKind::Exponential {
                    factor: parse_property_as_f64(value, "factor").unwrap(),
                },
                _ => todo!("kind {kind} not currently supported."),
            },
        }
    }

    fn parse_iter(value: &Value) -> CheckerKind {
        CheckerKind::Iter(parse_property_as_usize(value, "iter").unwrap())
    }

    fn parse_from(value: &Value) -> CheckerKind {
        let from = value.get("from").unwrap();

        CheckerKind::From {
            source: Self::parse_source(from),
            factor: Self::parse_factor(from),
            modulo: parse_property_as_usize(from, "modulo"),
        }
    }

    fn parse_kind(value: &Value) -> CheckerKind {
        let kind = value.get("type").unwrap().as_str().unwrap();

        match kind {
            "iter" => Self::parse_iter(value),
            "from" => Self::parse_from(value),
            _ => todo!("kind {kind} is not supported for checker."),
        }
    }

    pub fn from_value(value: &Value) -> Option<Self> {
        let checker = value.get("checker");

        if checker.is_none() {
            return None;
        }

        let checker = checker.unwrap();

        Some(Self {
            chance: parse_property_as_f64(checker, "chance")
                .unwrap_or(0.5.into())
                .into(),
            kind: Self::parse_kind(checker),
        })
    }

    pub fn to_tool(&self) -> Option<properties::CheckerType> {
        if !self.chance.roll() {
            None
        } else {
            Some({
                match &self.kind {
                    CheckerKind::Iter(n) => properties::CheckerType::Iter(n.generate()),
                    CheckerKind::From {
                        source,
                        factor,
                        modulo,
                    } => properties::CheckerType::From {
                        source: match &source.kind {
                            CheckerSourceKind::Center => properties::Source::Center,
                            CheckerSourceKind::Fixed { x, y } => {
                                properties::Source::Fixed(x.generate(), y.generate())
                            }
                        },
                        factor: match &factor.kind {
                            CheckerFactorKind::Linear => properties::Factor::Linear,
                            CheckerFactorKind::Exponential { factor } => {
                                properties::Factor::Exponential(factor.generate())
                            }
                        },
                        modulo: match &modulo {
                            None => None,
                            Some(v) => Some(v.generate()),
                        },
                    },
                }
            })
        }
    }
}
