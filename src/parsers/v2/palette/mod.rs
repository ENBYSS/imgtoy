use serde_yaml::Value;

use crate::parsers::v2::palette::{chroma::ChromaStrategy, hue::HueStrategy, lum::LumStrategy};

pub mod chroma;
pub mod hue;
pub mod lum;

pub struct MiscFlags {
    extremes: bool,
}

impl Default for MiscFlags {
    fn default() -> Self {
        Self { extremes: false }
    }
}

impl MiscFlags {
    pub fn from_value(value: &Value) -> Self {
        let mut default = Self::default();

        let raw_flags: Vec<&str> = value
            .get("misc-flags")
            .unwrap()
            .as_sequence()
            .unwrap()
            .iter()
            .map(|v| v.as_str().unwrap())
            .collect();

        if raw_flags.contains(&"extremes") {
            default.extremes = true;
        }

        default
    }
}

pub struct PaletteConfig {
    lum_strategy: LumStrategy,
    chroma_strategy: ChromaStrategy,
    hue_strategy: HueStrategy,
    misc_flag: MiscFlags,
}

impl PaletteConfig {
    fn parse_lum_strategy(value: &Value) -> LumStrategy {
        LumStrategy::from_value(value.get("lum-strategy").unwrap())
    }

    fn parse_chroma_strategy(value: &Value) -> ChromaStrategy {
        ChromaStrategy::from_value(value.get("chroma-strategy").unwrap())
    }

    fn parse_hue_strategy(value: &Value) -> HueStrategy {
        HueStrategy::from_value(value.get("hue-strategy").unwrap())
    }

    pub fn from_value(value: &Value) -> Self {
        Self {
            lum_strategy: Self::parse_lum_strategy(value),
            chroma_strategy: Self::parse_chroma_strategy(value),
            hue_strategy: Self::parse_hue_strategy(value),
            misc_flag: MiscFlags::from_value(value),
        }
    }
}

pub struct Palette {
    config: PaletteConfig,
}

impl Palette {
    pub fn from_value(value: &Value) -> Self {
        Self {
            config: PaletteConfig::from_value(value.get("palette").unwrap()),
        }
    }
}
