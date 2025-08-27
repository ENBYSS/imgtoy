use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, ValueProperty, Vf64, Vusize,
};

#[derive(Debug)]
pub enum LumStrategyKind {
    StackedExact { exact: Vec<Vf64> },
    Exact { exact: Vf64 },
    Random { count: Vusize, stack: bool },
    StackDistributed,
    StackDistributedArea { overlap: Vf64 },
    StackDistributedNudge { nudge_size: Vf64, per_lum: bool },
    LoopingPreference { segments: Vusize },
}

impl LumStrategyKind {
    pub fn generate(&self, hues: &Vec<f32>) -> Vec<(f32, f32)> {
        match self {
            Self::Exact { exact } => unimplemented!(),
            Self::StackedExact { exact } => unimplemented!(),
            Self::Random { count, stack } => unimplemented!(),
            Self::StackDistributed => unimplemented!(),
            Self::StackDistributedArea { overlap } => unimplemented!(),
            Self::StackDistributedNudge {
                nudge_size,
                per_lum,
            } => unimplemented!(),
            Self::LoopingPreference { segments } => unimplemented!(),
        }
    }

    fn parse_lum_list(value: &Value) -> Vec<Vf64> {
        value
            .get("lums")
            .unwrap()
            .as_sequence()
            .unwrap()
            .iter()
            .map(|l| ValueProperty::<f64>::property(l))
            .collect()
    }

    fn parse_count(value: &Value) -> Vusize {
        parse_property_as_usize(value, "count").unwrap()
    }

    pub fn parse_stacked_exact(value: &Value) -> Self {
        Self::StackedExact {
            exact: Self::parse_lum_list(value),
        }
    }

    pub fn parse_exact(value: &Value) -> Self {
        Self::Exact {
            exact: parse_property_as_f64(value, "lum").unwrap(),
        }
    }

    pub fn parse_random(value: &Value) -> Self {
        Self::Random {
            count: Self::parse_count(value),
            stack: value.get("stack").unwrap().as_bool().unwrap(),
        }
    }

    pub fn parse_stacked_distributed_area(value: &Value) -> Self {
        Self::StackDistributedArea {
            overlap: parse_property_as_f64(value, "overlap").unwrap(),
        }
    }

    pub fn parse_stacked_distributed_nudge(value: &Value) -> Self {
        Self::StackDistributedNudge {
            nudge_size: parse_property_as_f64(value, "nudge-size").unwrap(),
            per_lum: value.get("per-lum").unwrap().as_bool().unwrap(),
        }
    }

    pub fn parse_looping_preference(value: &Value) -> Self {
        Self::LoopingPreference {
            segments: parse_property_as_usize(value, "segments").unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct LumStrategy {
    kind: LumStrategyKind,
}

impl LumStrategy {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "stacked-exact" => LumStrategyKind::parse_stacked_exact(value),
            "exact" => LumStrategyKind::parse_exact(value),
            "random" => LumStrategyKind::parse_random(value),
            "distributed" => LumStrategyKind::StackDistributed,
            "distributed/area" => LumStrategyKind::parse_stacked_distributed_area(value),
            "distributed/nudge" => LumStrategyKind::parse_stacked_distributed_nudge(value),
            "looping-preference" => LumStrategyKind::parse_looping_preference(value),
            _ => todo!(),
        };

        Self { kind }
    }

    pub fn attach_lums(&self, hues: &Vec<f32>) -> Vec<(f32, f32)> {
        self.kind.generate(hues)
    }
}
