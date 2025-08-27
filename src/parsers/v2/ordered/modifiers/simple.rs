use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, Chance, ValueProperty, Vf64, Vusize,
};

#[derive(Debug)]
pub struct Invert {
    chance: Chance,
}

impl Invert {
    pub fn from_value(value: &Value) -> Option<Self> {
        let invert = value.get("invert");
        if invert.is_none() {
            return None;
        }
        let invert = invert.unwrap();

        let chance = parse_property_as_f64(invert, "chance")
            .unwrap_or(ValueProperty::Fixed(0.0))
            .into();

        Some(Invert { chance })
    }

    pub fn roll(&self) -> bool {
        self.chance.roll()
    }
}

#[derive(Debug)]
pub struct Exponentiate {
    chance: Chance,
    factor: Vf64,
}

impl Exponentiate {
    pub fn from_value(value: &Value) -> Option<Self> {
        let exponentiate = value.get("exponentiate");
        if exponentiate.is_none() {
            return None;
        }
        let exponentiate = exponentiate.unwrap();

        let chance = parse_property_as_f64(exponentiate, "chance")
            .unwrap_or(ValueProperty::Fixed(0.0))
            .into();
        let factor =
            parse_property_as_f64(exponentiate, "factor").unwrap_or(ValueProperty::Fixed(0.0));

        Some(Exponentiate { chance, factor })
    }

    pub fn generate_factor(&self) -> Option<f64> {
        if self.chance.roll() {
            Some(self.factor.generate())
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct Blur {
    chance: Chance,
    factor: Vusize,
}

impl Blur {
    pub fn from_value(value: &Value) -> Option<Self> {
        let blur = value.get("blur");
        if blur.is_none() {
            return None;
        }
        let blur = blur.unwrap();

        let chance = parse_property_as_f64(blur, "chance")
            .unwrap_or(ValueProperty::Fixed(0.0))
            .into();
        let factor = parse_property_as_usize(blur, "factor").unwrap_or(ValueProperty::Fixed(1));

        Some(Blur { chance, factor })
    }

    pub fn generate_factor(&self) -> Option<usize> {
        if self.chance.roll() {
            Some(self.factor.generate())
        } else {
            None
        }
    }
}
