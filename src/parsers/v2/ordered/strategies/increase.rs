use std::collections::HashMap;

use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, ValueProperty, Vf64, Vusize,
};

#[derive(PartialEq, Eq, Hash)]
pub enum IncreaseValueKind {
    Linear(Vusize),
    Exponential(Vusize),
}

pub enum IncreaseKind {
    Exact(IncreaseValueKind),
    Ratios(HashMap<IncreaseValueKind, f64>),
}

pub struct Increase {
    kind: IncreaseKind,
    chance: Vf64,
}

impl Default for Increase {
    fn default() -> Self {
        Increase {
            kind: IncreaseKind::Exact(IncreaseValueKind::Linear(ValueProperty::Fixed(1))),
            chance: ValueProperty::Fixed(0.0),
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

            let mut ratios = HashMap::new();
            if let Some(linear) = linear {
                ratios.insert(IncreaseValueKind::Linear(factor.clone()), linear);
            }
            if let Some(exponential) = exponential {
                ratios.insert(IncreaseValueKind::Exponential(factor), exponential);
            }

            Increase { kind: IncreaseKind::Ratios(ratios), chance }
        },
        Value::String(strategy_type) => {
            let kind = match strategy_type.as_str() {
                "linear" => IncreaseValueKind::Linear(factor),
                "exponential" => IncreaseValueKind::Exponential(factor),
                _ => panic!("[ordered.increase-strategy.type] must be 'linear' or 'exponential"),
            };

            Increase { kind: IncreaseKind::Exact(kind), chance }
        },
        _ => panic!("[ordered.orientation] must be a mapping of ratios, or one of 'down-right' / 'up-right'")
    };

        Some(increase)
    }
}
