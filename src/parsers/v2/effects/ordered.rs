use image_effects::dither::ordered::{self, OrderedStrategy};
use rand::seq::IndexedRandom;
use serde_yaml::Value;

use crate::parsers::v2::{
    ordered::{
        modifiers::{
            checker::Checker,
            mirror::Mirror,
            rotation::Rotation,
            simple::{Blur, Exponentiate, Invert},
        },
        strategies::Effect,
    },
    palette::{self, Palette},
};

#[derive(Debug)]
/// Represents dithering using the Ordered strategy.
pub struct Ordered {
    /// A list of strategies listed under this.
    /// Only one of these will be selected.
    strategies: Vec<Effect>,
    blur: Option<Blur>,
    exponentiate: Option<Exponentiate>,
    rotation: Option<Rotation>,
    checker: Option<Checker>,
    invert: Option<Invert>,
    mirror: Option<Mirror>,
    palette: palette::Palette,
}

impl Ordered {
    pub fn from_value(value: &Value) -> Self {
        let value = value.get("ordered").unwrap();

        Self {
            strategies: value
                .get("strategies")
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .map(|e| Effect::from_value(e))
                .collect(),
            blur: Blur::from_value(value),
            exponentiate: Exponentiate::from_value(value),
            rotation: Rotation::from_value(value),
            checker: Checker::from_value(value),
            invert: Invert::from_value(value),
            mirror: Mirror::from_value(value),
            palette: Palette::from_value(value),
        }
    }

    pub fn generate_effect(&self) -> ordered::Ordered {
        let mut strategy: OrderedStrategy = self
            .strategies
            .choose(&mut rand::rng())
            .unwrap()
            .generate_effect();

        if let Some(blur) = &self.blur {
            if let Some(blur) = blur.generate_factor() {
                strategy = strategy.blur(blur);
            }
        }
        if let Some(exponentiate) = &self.exponentiate {
            if let Some(exponentiate) = exponentiate.generate_factor() {
                strategy = strategy.exponentiate(exponentiate);
            }
        }
        if let Some(rotation) = &self.rotation {
            if let Some(rotation) = rotation.to_tool() {
                strategy = strategy.rotate(rotation);
            }
        }
        if let Some(checker) = &self.checker {
            if let Some(checker) = checker.to_tool() {
                strategy = strategy.checker(checker);
            }
        }
        if let Some(invert) = &self.invert {
            if invert.roll() {
                strategy = strategy.invert();
            }
        }
        if let Some(mirror) = &self.mirror {
            for line in mirror.to_tool() {
                strategy = strategy.mirror(line);
            }
        }

        ordered::Ordered::new(self.palette.generate(), strategy)
    }
}
