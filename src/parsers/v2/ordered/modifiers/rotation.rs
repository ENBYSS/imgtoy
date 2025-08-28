use std::marker::PhantomData;

use image_effects::dither::{
    self,
    ordered::tools::{self, properties},
};
use rand::seq::IndexedRandom;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_f64, Chance, ValueProperty, Vf64};

#[derive(Debug)]
pub enum RotationDirection {
    RIGHT,
    LEFT,
    HALF,
    NONE,
}

impl RotationDirection {
    pub fn to_tool(&self) -> properties::Rotation {
        match self {
            Self::RIGHT => properties::Rotation::Right,
            Self::LEFT => properties::Rotation::Left,
            Self::HALF => properties::Rotation::Half,
            Self::NONE => properties::Rotation::None,
        }
    }
}

#[derive(Debug)]
pub struct Rotation {
    chance: Chance,
    values: Vec<RotationDirection>,
}

impl Rotation {
    pub fn from_value(value: &Value) -> Option<Self> {
        let rotation = value.get("rotation");
        if rotation.is_none() {
            return None;
        }
        let rotation = rotation.unwrap();

        let chance = parse_property_as_f64(rotation, "chance")
            .unwrap_or(ValueProperty::Fixed(0.0))
            .into();

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
                _ => unimplemented!("rotation direction {prop} is not supported."),
            })
            .collect::<Vec<_>>();

        Some(Rotation { chance, values })
    }

    pub fn to_tool(&self) -> Option<properties::Rotation> {
        if !self.chance.roll() {
            None
        } else {
            Some(self.values.choose(&mut rand::rng()).unwrap().to_tool())
        }
    }
}
