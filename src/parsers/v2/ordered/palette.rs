use crate::parsers::v2::structure::Vusize;

pub enum PaletteKind {
    RandomV2,
}

pub enum LumStrategyKind {}

pub struct LumStrategy {
    count: Vusize,
    kind: LumStrategyKind,
}

pub enum ChromaStrategyKind {}

pub struct ChromaStrategy {
    kind: ChromaStrategyKind,
}

pub enum HueStrategyKind {}

pub struct HueStrategy {
    kind: HueStrategyKind,
}

pub struct MiscFlags {
    flags: Vec<String>,
}

pub struct PaletteConfig {
    lum_strategy: LumStrategy,
    chroma_strategy: ChromaStrategy,
    hue_strategy: HueStrategy,
    misc_flag: MiscFlags,
}

pub struct Palette {
    kind: PaletteKind,
    config: PaletteConfig,
}
