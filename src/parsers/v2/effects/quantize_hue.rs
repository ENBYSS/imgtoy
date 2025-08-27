use image_effects::filter::filters;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{ValueProperty, Vf64};

#[derive(Debug)]
pub struct QuantizeHue {
    hues: Vec<Vf64>,
}

impl QuantizeHue {
    pub fn from_value(value: &Value) -> Self {
        let hues: Vec<Vf64> = value
            .get("hues")
            .unwrap()
            .as_sequence()
            .unwrap()
            .iter()
            .map(|h| ValueProperty::<f64>::property(h))
            .collect();

        QuantizeHue { hues }
    }

    pub fn generate(&self) -> filters::QuantizeHue {
        filters::QuantizeHue::with_hues(
            self.hues
                .iter()
                .map(|value| value.generate() as f32)
                .collect(),
        )
    }
}
