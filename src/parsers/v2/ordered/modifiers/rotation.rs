use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_f64, ValueProperty, Vf64};

pub enum RotationDirection {
    RIGHT,
    LEFT,
    HALF,
    NONE,
}

pub struct Rotation {
    chance: Vf64,
    values: Vec<RotationDirection>,
}

impl Rotation {
    pub fn from_value(value: &Value) -> Option<Self> {
        let rotation = value.get("rotation");
        if rotation.is_none() {
            return None;
        }
        let rotation = rotation.unwrap();

        let chance = parse_property_as_f64(rotation, "chance").unwrap_or(ValueProperty::Fixed(0.0));

        let values = rotation
            .get("values")
            .expect("expected [values]")
            .as_sequence()
            .expect("expected rot-seq")
            .iter()
            .map(|prop| prop.as_str().expect("expected str"))
            .map(|prop| match prop {
                "right" => RotationDirection::RIGHT,
                "left" => RotationDirection::LEFT,
                "half" => RotationDirection::HALF,
                "none" => RotationDirection::NONE,
                _ => todo!(),
            })
            .collect::<Vec<_>>();

        Some(Rotation { chance, values })
    }
}
