use rand::Rng;
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
            _ => unimplemented!("hue distribution {value} is not supported."),
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

    fn generate_hue_neighbourhood(hue: f64, size: f64, n: u64, dist: &HueDistribution) -> Vec<f32> {
        let mut neighbourhood = Vec::new();

        let lower_end = hue as f32 - size as f32;
        let upper_end = hue as f32 + size as f32;

        for i in 0..n {
            match dist {
                HueDistribution::Linear => {
                    let fraction = (i as f32) / ((n - 1) as f32);
                    neighbourhood.push(lower_end + (size as f32 * 2.0 * fraction));
                }
                HueDistribution::Random => {
                    neighbourhood.push(rand::rng().random_range(lower_end..upper_end));
                }
            }
        }

        neighbourhood
    }

    pub fn _execute(&self) -> Vec<f32> {
        self.execute_with_seed_hue(rand::rng().random_range(0.0..360.0))
    }

    pub fn execute_with_seed_hue(&self, seed_hue: f64) -> Vec<f32> {
        match self {
            Self::Neighbour {
                size,
                count,
                distribution,
            } => Self::generate_hue_neighbourhood(
                seed_hue,
                size.generate(),
                count.generate() as u64,
                distribution,
            ),
            Self::Contrast {
                size,
                count,
                distribution,
            } => Self::generate_hue_neighbourhood(
                seed_hue + 180.0,
                size.generate(),
                count.generate() as u64,
                distribution,
            ),
            Self::Penpal {
                size,
                count,
                distribution,
                distance,
            } => Self::generate_hue_neighbourhood(
                seed_hue + distance.generate(),
                size.generate(),
                count.generate() as u64,
                distribution,
            ),
            Self::Cycle { count } => {
                let count = count.generate();
                (1..=count)
                    .map(|i| seed_hue as f32 + i as f32 * (360.0 / (count as f32 + 1.0)))
                    .collect()
            }
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
            _ => unimplemented!("hue-strategy {kind} is not supported"),
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

    pub fn generate_hues(&self) -> Vec<f32> {
        let seed_hue = rand::rng().random_range(0.0..360.0);

        self.kinds
            .iter()
            .flat_map(|strategy| strategy.execute_with_seed_hue(seed_hue))
            .collect()
    }
}
