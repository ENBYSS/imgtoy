use serde_yaml::Value;

use crate::parsers::v2::{
    effects::Effects,
    ordered::{
        self,
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

/// Represents dithering using the Ordered strategy.
pub struct Ordered {
    /// A list of strategies listed under this.
    /// Only one of these will be selected.
    strategies: Effects,
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
        Self {
            strategies: Effects::from_value(value),
            blur: Blur::from_value(value),
            exponentiate: Exponentiate::from_value(value),
            rotation: Rotation::from_value(value),
            checker: Checker::from_value(value),
            invert: Invert::from_value(value),
            mirror: Mirror::from_value(value),
            palette: Palette::from_value(value),
        }
    }
}
