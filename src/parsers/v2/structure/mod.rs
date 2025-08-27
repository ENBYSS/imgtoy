use serde_yaml::Value;

use crate::parsers::v2::{
    effects::Effects,
    structure::meta::{Output, Source},
};

pub mod meta;
pub mod value;

#[derive(Debug)]
pub struct MainConfiguration {
    source: Source,
    output: Output,
    effects: Effects,
}

impl MainConfiguration {
    pub fn from_value(value: &Value) -> Self {
        Self {
            source: Source::from_value(value),
            output: Output::from_value(value),
            effects: Effects::from_value(value),
        }
    }
}
