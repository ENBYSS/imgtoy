use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, Vf64, Vusize,
};

#[derive(Debug)]
pub enum HueDistribution {
    Linear,
    Random,
}

impl From<&str> for HueDistribution {
    fn from(value: &str) -> Self {
        match value {
            "linear" => Self::Linear,
            "random" => Self::Random,
            _ => todo!(),
        }
    }
}

#[derive(Debug)]
pub enum HueStrategyKind {
    Neighbour {
        size: Vf64,
        count: Vusize,
        distribution: HueDistribution,
    },
    Contrast {
        size: Vf64,
        count: Vusize,
        distribution: HueDistribution,
    },
    Penpal {
        size: Vf64,
        count: Vusize,
        distribution: HueDistribution,
        distance: Vf64,
    },
    Cycle {
        count: Vusize,
    },
}

impl HueStrategyKind {
    fn parse_size(value: &Value) -> Vf64 {
        parse_property_as_f64(value, "size").unwrap()
    }

    fn parse_count(value: &Value) -> Vusize {
        parse_property_as_usize(value, "count").unwrap()
    }

    fn parse_distribution(value: &Value) -> HueDistribution {
        value.get("distribution").unwrap().as_str().unwrap().into()
    }

    fn parse_distance(value: &Value) -> Vf64 {
        parse_property_as_f64(value, "distance").unwrap()
    }

    pub fn parse_neighbour(value: &Value) -> Self {
        Self::Neighbour {
            size: Self::parse_size(value),
            count: Self::parse_count(value),
            distribution: Self::parse_distribution(value),
        }
    }

    pub fn parse_contrast(value: &Value) -> Self {
        Self::Contrast {
            size: Self::parse_size(value),
            count: Self::parse_count(value),
            distribution: Self::parse_distribution(value),
        }
    }

    pub fn parse_penpal(value: &Value) -> Self {
        Self::Penpal {
            size: Self::parse_size(value),
            count: Self::parse_count(value),
            distribution: Self::parse_distribution(value),
            distance: Self::parse_distance(value),
        }
    }

    pub fn parse_cycle(value: &Value) -> Self {
        Self::Cycle {
            count: Self::parse_count(value),
        }
    }
}

#[derive(Debug)]
pub struct HueStrategies {
    kinds: Vec<HueStrategyKind>,
}

impl HueStrategies {
    fn parse_hue_strategy(value: &Value) -> HueStrategyKind {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "neighbour" => HueStrategyKind::parse_neighbour(value),
            "contrast" => HueStrategyKind::parse_contrast(value),
            "penpal" => HueStrategyKind::parse_penpal(value),
            "cycle" => HueStrategyKind::parse_cycle(value),
            _ => todo!(),
        };

        kind
    }

    pub fn from_value(value: &Value) -> Self {
        Self {
            kinds: value
                .as_sequence()
                .unwrap()
                .iter()
                .map(|s| Self::parse_hue_strategy(s))
                .collect(),
        }
    }
}
