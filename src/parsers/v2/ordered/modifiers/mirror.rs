use crate::parsers::v2::structure::Vf64;

pub enum MirrorDirection {
    DOWNRIGHT,
    UPRIGHT,
    HORIZONTAL,
    VERTICAL,
}

pub struct Mirror {
    flip: Vf64,
    thorough: Vf64,
    chance: Vf64,
    directions: Vec<Vec<MirrorDirection>>,
}
