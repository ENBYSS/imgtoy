use rand::Rng;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, ValueProperty, Vf64, Vusize,
};

#[derive(Debug)]
pub enum LumStrategyKind {
    StackedExact {
        stacks: Vusize,
        exact: Vec<Vf64>,
    },
    Exact {
        exact: Vf64,
    },
    Random {
        count: Vusize,
        stack: bool,
    },
    StackDistributed {
        stacks: Vusize,
    },
    StackDistributedArea {
        stacks: Vusize,
        overlap: Vf64,
    },
    StackDistributedNudge {
        stacks: Vusize,
        nudge_size: Vf64,
        per_lum: bool,
    },
    LoopingPreference {
        segments: Vusize,
    },
}

impl LumStrategyKind {
    pub fn generate(&self, hues: &Vec<f32>, min_lum: f64, max_lum: f64) -> Vec<(f32, f32)> {
        match self {
            Self::Exact { exact } => todo!("implement exact strategy"),
            Self::StackedExact { stacks, exact } => todo!("implement stacked exact strategy"),
            Self::Random { count, stack } => todo!("implement random strategy"),
            Self::StackDistributed { stacks } => todo!("implement stack distributed strategy"),
            Self::StackDistributedArea { stacks, overlap } => {
                let mut cols = Vec::new();
                for hue in hues.iter() {
                    let stacks = stacks.generate();
                    for mut i in 0..stacks {
                        let span_size = max_lum - min_lum;
                        let step_size = span_size / stacks as f64;

                        let mut area_start = min_lum + (i as f64 * step_size);
                        let mut area_end = area_start + step_size;

                        let overlap = overlap.generate();

                        area_start = (area_start - overlap).max(min_lum);
                        area_end = (area_end + overlap).min(max_lum);

                        let l = rand::rng().random_range(area_start..area_end) as f32;
                        cols.push((l, *hue));
                    }
                }
                cols
            }
            Self::StackDistributedNudge {
                nudge_size,
                per_lum,
                stacks,
            } => unimplemented!(),
            Self::LoopingPreference { segments } => unimplemented!(),
        }
    }

    fn parse_stacks(value: &Value) -> Vusize {
        parse_property_as_usize(value, "count").unwrap()
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
            stacks: Self::parse_stacks(value),
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

    pub fn parse_stacked_distributed(value: &Value) -> Self {
        Self::StackDistributed {
            stacks: Self::parse_stacks(value),
        }
    }

    pub fn parse_stacked_distributed_area(value: &Value) -> Self {
        Self::StackDistributedArea {
            overlap: parse_property_as_f64(value, "overlap").unwrap(),
            stacks: Self::parse_stacks(value),
        }
    }

    pub fn parse_stacked_distributed_nudge(value: &Value) -> Self {
        Self::StackDistributedNudge {
            nudge_size: parse_property_as_f64(value, "nudge-size").unwrap(),
            per_lum: value.get("per-lum").unwrap().as_bool().unwrap(),
            stacks: Self::parse_stacks(value),
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
    min_lum: Option<Vf64>,
    max_lum: Option<Vf64>,
}

impl LumStrategy {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "stacked-exact" => LumStrategyKind::parse_stacked_exact(value),
            "exact" => LumStrategyKind::parse_exact(value),
            "random" => LumStrategyKind::parse_random(value),
            "distributed" => LumStrategyKind::parse_stacked_distributed(value),
            "distributed/area" => LumStrategyKind::parse_stacked_distributed_area(value),
            "distributed/nudge" => LumStrategyKind::parse_stacked_distributed_nudge(value),
            "looping-preference" => LumStrategyKind::parse_looping_preference(value),
            _ => unimplemented!("lum-strategy '{kind}' is not supported"),
        };

        Self {
            kind,
            min_lum: parse_property_as_f64(value, "min-lum"),
            max_lum: parse_property_as_f64(value, "max-lum"),
        }
    }

    pub fn attach_lums(&self, hues: &Vec<f32>) -> Vec<(f32, f32)> {
        self.kind.generate(
            hues,
            self.min_lum.as_ref().map(|l| l.generate()).unwrap_or(0.0),
            self.max_lum.as_ref().map(|l| l.generate()).unwrap_or(100.0),
        )
    }
}
