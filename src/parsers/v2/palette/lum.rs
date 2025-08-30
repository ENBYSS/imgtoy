use rand::Rng;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, ValueProperty, Vf64, Vusize,
};

#[derive(Debug)]
pub enum LumStrategyKind {
    StackedExact {
        exact: Vec<Vf64>,
    },
    Exact {
        exact: Vf64,
    },
    Random {
        stacks: Option<Vusize>,
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
    },
    LoopingPreference {
        focus_hue: Vf64,
        segments: Vusize,
        spread: (Vusize, Vf64),
        clamp: (Vf64, Vf64),
    },
}

impl LumStrategyKind {
    pub fn generate(&self, hues: &Vec<f32>, min_lum: f64, max_lum: f64) -> Vec<(f32, f32)> {
        match self {
            Self::Exact { exact } => hues
                .iter()
                .map(|hue| (exact.generate() as f32, *hue))
                .collect(),
            Self::StackedExact { exact } => hues
                .iter()
                .flat_map(|hue| exact.iter().map(|exact| (exact.generate() as f32, *hue)))
                .collect(),
            Self::Random { stacks } => {
                let mut rng = rand::rng();
                let stacks = stacks.as_ref().map(|v| v.generate()).unwrap_or(1);

                let mut colours = Vec::with_capacity(stacks * hues.len());
                for hue in hues {
                    for _ in 0..stacks {
                        colours.push((rng.random_range(0.0..100.0) as f32, *hue));
                    }
                }
                colours
            }
            Self::StackDistributed { stacks } => {
                let stacks = stacks.generate();
                let mut cols = Vec::with_capacity(stacks * hues.len());

                for hue in hues.iter() {
                    for i in 0..stacks {
                        let span_size = max_lum - min_lum;
                        let l = min_lum + (i as f64 / (stacks as f64 - 1.0)) * span_size;
                        cols.push((l as f32, *hue));
                    }
                }

                cols
            }
            Self::StackDistributedArea { stacks, overlap } => {
                let stacks = stacks.generate();
                let mut cols = Vec::with_capacity(stacks * hues.len());

                for hue in hues.iter() {
                    for i in 0..stacks {
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
            Self::StackDistributedNudge { nudge_size, stacks } => {
                let mut rng = rand::rng();
                let stacks = stacks.generate();
                let mut cols = Vec::with_capacity(stacks * hues.len());
                let nudge_size = nudge_size.generate();

                for hue in hues.iter() {
                    for i in 0..stacks {
                        let span_size = max_lum - min_lum;
                        let mut l = min_lum + (i as f64 / (stacks as f64 - 1.0)) * span_size;

                        l = (l + rng.random_range((-nudge_size)..nudge_size)).clamp(0.0, 100.0);

                        cols.push((l as f32, *hue));
                    }
                }

                cols
            }
            Self::LoopingPreference {
                focus_hue,
                segments,
                spread: (s_amnt, s_size),
                clamp: (c_min, c_max),
            } => {
                // **|(HUE - TARGET / (360 / N)) % 2 - 1.0|**
                let focus_hue = focus_hue.generate() % 360.0;
                let segments = segments.generate();
                let s_amnt = s_amnt.generate();
                let s_size = s_size.generate();
                let c_min = c_min.generate();
                let c_max = c_max.generate();
                let mut rng = rand::rng();

                let mut cols = Vec::with_capacity(hues.len() * (s_amnt + 1));

                for hue in hues.iter() {
                    let diff = (focus_hue - *hue as f64).abs();
                    let segment_size = 360.0 / segments as f64;
                    let segment_loc = (diff / segment_size) % 2.0;
                    let final_lum = (segment_loc - 1.0).abs() as f32;
                    cols.push((final_lum * 100.0, *hue));

                    for _ in 0..s_amnt {
                        let (mut r_min, mut r_max) = (
                            (final_lum as f64 - s_size).max(c_min),
                            (final_lum as f64 + s_size).min(c_max),
                        );

                        if (final_lum as f64) < r_min {
                            r_min = final_lum as f64;
                        }
                        if (final_lum as f64) > r_max {
                            r_max = final_lum as f64;
                        }

                        r_min *= 100.0;
                        r_max *= 100.0;

                        if r_min < r_max {
                            cols.push((rng.random_range(r_min..r_max) as f32, *hue))
                        }
                        // else {
                        //     println!("final lum: {final_lum}, spread size: {s_size}, clamp: {c_min} ~ {c_max}, range: {r_min} ~ {r_max}");
                        // }
                    }
                }

                // {
                //     println!("HUES: {hues:?}");
                //     println!(
                //         "LUMS: {:?}",
                //         cols.iter().map(|(l, _)| *l).collect::<Vec<f32>>()
                //     )
                // }

                cols
            }
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
            stacks: parse_property_as_usize(value, "stacks"),
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
            stacks: Self::parse_stacks(value),
        }
    }

    pub fn parse_looping_preference(value: &Value) -> Self {
        Self::LoopingPreference {
            focus_hue: parse_property_as_f64(value, "focus-hue").unwrap(),
            segments: parse_property_as_usize(value, "segments").unwrap(),
            spread: (
                parse_property_as_usize(value, "spread-amnt").unwrap_or(0.into()),
                parse_property_as_f64(value, "spread-size").unwrap_or(10.0.into()),
            ),
            clamp: (
                parse_property_as_f64(value, "clamp-min").unwrap_or(0.0.into()),
                parse_property_as_f64(value, "clamp-max").unwrap_or(100.0.into()),
            ),
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
