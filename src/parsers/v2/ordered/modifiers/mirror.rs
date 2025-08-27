use image_effects::dither::ordered::tools::mirror::{self, MirrorLine};
use rand::seq::IndexedRandom;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_f64, Chance, ValueProperty, Vf64};

#[derive(Debug, Clone, Copy)]
pub enum MirrorDirection {
    Downright,
    Upright,
    Vertical,
    Horizontal,
}

impl Into<mirror::MirrorDirection> for MirrorDirection {
    fn into(self) -> mirror::MirrorDirection {
        match self {
            MirrorDirection::Downright => mirror::MirrorDirection::Downright,
            MirrorDirection::Upright => mirror::MirrorDirection::Upright,
            MirrorDirection::Horizontal => mirror::MirrorDirection::Horizontal,
            MirrorDirection::Vertical => mirror::MirrorDirection::Vertical,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Mirror {
    flip: Chance,
    thorough: Chance,
    chance: Chance,
    directions: Vec<Vec<MirrorDirection>>,
}

impl Mirror {
    pub fn from_value(value: &Value) -> Option<Self> {
        let mirror = value.get("mirror");
        if mirror.is_none() {
            return None;
        }
        let mirror = mirror.unwrap();

        let directions = mirror
            .get("directions")
            .expect("[mirror.directions] must specify at least one direction.")
            .as_sequence()
            .expect("[mirror.directions] must be a list.");

        let flip = parse_property_as_f64(mirror, "flip").unwrap_or(ValueProperty::Fixed(0.0));
        let thorough =
            parse_property_as_f64(mirror, "thorough").unwrap_or(ValueProperty::Fixed(0.0));
        let chance = parse_property_as_f64(mirror, "chance").unwrap_or(ValueProperty::Fixed(0.0));

        let directions = directions
            .iter()
            .map(|direction_set| {
                let direction_set = direction_set
                    .as_sequence()
                    .expect("[mirror.directions[$]] should be a sequence of strings.");
                direction_set
                    .iter()
                    .map(|entry| {
                        let direction_name = entry
                            .as_str()
                            .expect("[mirror.directions[$][$]] must be a string.");

                        match direction_name {
                            "downright" => MirrorDirection::Downright,
                            "upright" => MirrorDirection::Upright,
                            "horizontal" => MirrorDirection::Horizontal,
                            "vertical" => MirrorDirection::Vertical,
                            _ => todo!(),
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>();

        Some(Mirror {
            flip: flip.into(),
            thorough: thorough.into(),
            chance: chance.into(),
            directions,
        })
    }

    pub fn generate(&self) -> Vec<MirrorLine> {
        if !self.chance.roll() {
            return vec![];
        }

        let mirror = self.directions.choose(&mut rand::rng()).unwrap();

        mirror
            .iter()
            .map(|direction| MirrorLine {
                direction: direction.clone().into(),
                flip: self.flip.roll(),
                thorough: self.thorough.roll(),
            })
            .collect()
    }
}
