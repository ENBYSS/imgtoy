use image_effects::filter::filters;
use serde_yaml::Value;

use crate::parsers::v2::structure::value::{parse_property_as_f64, ValueProperty};

#[derive(Debug)]
/// Represents a brighten effect.
pub struct Brighten(ValueProperty<f64>);

impl Brighten {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("brighten").unwrap(), "factor").unwrap())
    }
}

#[derive(Debug)]
/// Represents a saturation effect.
pub struct Saturate(ValueProperty<f64>);

impl Saturate {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("saturate").unwrap(), "factor").unwrap())
    }
}

#[derive(Debug)]
/// Represents a constrast effect.
pub struct Contrast(ValueProperty<f64>);

impl Contrast {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("contrast").unwrap(), "factor").unwrap())
    }

    pub fn generate(&self) -> filters::Contrast {
        filters::Contrast(self.0.generate() as f32)
    }
}

#[derive(Debug)]
/// Represents a hue rotation effect.
pub struct HueRotate(ValueProperty<f64>);

impl HueRotate {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("hue-rotate").unwrap(), "factor").unwrap())
    }

    pub fn generate(&self) -> filters::HueRotate {
        filters::HueRotate(self.0.generate() as f32)
    }
}

#[derive(Debug)]
pub struct MultiplyHue(ValueProperty<f64>);

impl MultiplyHue {
    pub fn from_value(value: &Value) -> Self {
        Self(parse_property_as_f64(value.get("multiply-hue").unwrap(), "factor").unwrap())
    }

    pub fn generate(&self) -> filters::MultiplyHue {
        filters::MultiplyHue(self.0.generate() as f32)
    }
}
