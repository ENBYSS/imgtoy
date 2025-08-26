use crate::parsers::v2::{ordered, structure::ValueProperty};

pub struct Brighten(ValueProperty<f64>);
pub struct Saturate(ValueProperty<f64>);
pub struct Contrast(ValueProperty<f64>);
pub struct HueRotate(ValueProperty<f64>);

pub struct Ordered {
    strategies: Vec<ordered::strategies::Effect>,
    blur: ordered::modifiers::simple::Blur,
    exponentiate: ordered::modifiers::simple::Exponentiate,
    rotation: ordered::modifiers::rotation::Rotation,
    checker: ordered::modifiers::checker::Checker,
    invert: ordered::modifiers::simple::Invert,
    mirror: ordered::modifiers::mirror::Mirror,
    palette: ordered::palette::Palette,
}
