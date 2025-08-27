use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_f64, ValueProperty};

/// Represents a brighten effect.
pub struct Brighten(ValueProperty<f64>);

impl Brighten {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("brighten").unwrap(), "factor").unwrap())
    }
}

/// Represents a saturation effect.
pub struct Saturate(ValueProperty<f64>);

impl Saturate {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("saturate").unwrap(), "factor").unwrap())
    }
}

/// Represents a constrast effect.
pub struct Contrast(ValueProperty<f64>);

impl Contrast {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("contrast").unwrap(), "factor").unwrap())
    }
}

/// Represents a hue rotation effect.
pub struct HueRotate(ValueProperty<f64>);

impl HueRotate {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("hue-rotate").unwrap(), "factor").unwrap())
    }
}

pub struct MultiplyHue(ValueProperty<f64>);

impl MultiplyHue {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("multiply-hue").unwrap(), "factor").unwrap())
    }
}
