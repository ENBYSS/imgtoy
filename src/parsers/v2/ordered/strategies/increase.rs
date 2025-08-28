use image_effects::dither::ordered::algorithms::properties;
use rand::Rng;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, Chance, ValueProperty, Vusize,
};

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum IncreaseValueKind {
    Linear(Vusize),
    Exponential(Vusize),
}

impl IncreaseValueKind {
    pub fn to_property(&self) -> properties::Increase {
        match self {
            Self::Linear(f) => properties::Increase::Linear(f.generate() as u8),
            Self::Exponential(f) => properties::Increase::Exponential(f.generate() as u8),
        }
    }
}

#[derive(Debug)]
pub enum IncreaseKind {
    Exact(IncreaseValueKind),
    Ratios(Vec<(f64, IncreaseValueKind)>),
}

#[derive(Debug)]
pub struct Increase {
    kind: IncreaseKind,
    chance: Chance,
}

impl Default for Increase {
    fn default() -> Self {
        Increase {
            kind: IncreaseKind::Exact(IncreaseValueKind::Linear(ValueProperty::Fixed(1))),
            chance: ValueProperty::Fixed(0.0).into(),
        }
    }
}

impl Increase {
    pub fn from_value(value: &Value) -> Option<Self> {
        let increase = value.get("increase");
        if increase.is_none() {
            return None;
        }
        let increase = increase.unwrap();

        let strategy_type = increase
            .get("type")
            .expect("[ordered.strategy.increase-strategy] must have a [type]");

        let chance = parse_property_as_f64(increase, "chance").unwrap();
        let factor = parse_property_as_usize(increase, "factor").unwrap();

        let increase = match strategy_type {
        Value::Mapping(mapping) => {
            let linear = mapping.get("linear").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.horizontal] must be a float.".to_string()) }));
            let exponential = mapping.get("exponential").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.vertical] must be a float.".to_string()) }));

            let mut ratios = Vec::new();
            if let Some(linear) = linear {
                ratios.push((linear, IncreaseValueKind::Linear(factor.clone())));
            }
            if let Some(exponential) = exponential {
                ratios.push((exponential, IncreaseValueKind::Exponential(factor)));
            }

            Increase { kind: IncreaseKind::Ratios(ratios), chance: chance.into() }
        },
        Value::String(strategy_type) => {
            let kind = match strategy_type.as_str() {
                "linear" => IncreaseValueKind::Linear(factor),
                "exponential" => IncreaseValueKind::Exponential(factor),
                _ => panic!("[ordered.increase-strategy.type] must be 'linear' or 'exponential"),
            };

            Increase { kind: IncreaseKind::Exact(kind), chance: chance.into() }
        },
        _ => panic!("[ordered.orientation] must be a mapping of ratios, or one of 'down-right' / 'up-right'")
    };

        Some(increase)
    }

    pub fn generate(&self) -> properties::Increase {
        if !self.chance.roll() {
            todo!("exit")
        }

        match &self.kind {
            IncreaseKind::Exact(increase) => increase.to_property(),
            IncreaseKind::Ratios(ratios) => {
                let capacity = ratios.iter().map(|(ratio, _)| ratio).sum();

                let mut flag = rand::rng().random_range(0.0..capacity);

                for (ratio, increase) in ratios {
                    flag -= ratio;
                    if flag <= 0.0 {
                        return increase.to_property();
                    }
                }

                todo!("fix ratio calculation")
            }
        }
    }
}
