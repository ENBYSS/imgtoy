use crate::parsers::v2::structure::Vf64;

pub enum RotationDirection {
    RIGHT,
    LEFT,
    HALF,
}

pub struct Rotation {
    chance: Vf64,
    values: Vec<RotationDirection>,
}
