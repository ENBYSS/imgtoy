use palette::{IntoColor, Lch, Srgb};
use serde_yaml::Value;

use crate::parsers::v2::palette::{chroma::ChromaStrategy, hue::HueStrategies, lum::LumStrategy};

pub mod chroma;
pub mod hue;
pub mod lum;

#[derive(Debug)]
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

#[derive(Debug)]
pub struct PaletteConfig {
    lum_strategy: LumStrategy,
    chroma_strategy: ChromaStrategy,
    hue_strategies: HueStrategies,
    misc_flag: MiscFlags,
}

impl PaletteConfig {
    fn parse_lum_strategy(value: &Value) -> LumStrategy {
        LumStrategy::from_value(value.get("lum-strategy").unwrap())
    }

    fn parse_chroma_strategy(value: &Value) -> ChromaStrategy {
        ChromaStrategy::from_value(value.get("chroma-strategy").unwrap())
    }

    fn parse_hue_strategy(value: &Value) -> HueStrategies {
        HueStrategies::from_value(value.get("hue-strategies").unwrap())
    }

    pub fn from_value(value: &Value) -> Self {
        Self {
            lum_strategy: Self::parse_lum_strategy(value),
            chroma_strategy: Self::parse_chroma_strategy(value),
            hue_strategies: Self::parse_hue_strategy(value),
            misc_flag: MiscFlags::from_value(value),
        }
    }
}

#[derive(Debug)]
pub struct Palette {
    config: PaletteConfig,
}

impl Palette {
    pub fn from_value(value: &Value) -> Self {
        let value = value.get("palette").unwrap();
        let value = value.get("config").unwrap();
        Self {
            config: PaletteConfig::from_value(value),
        }
    }

    pub fn generate(&self) -> Vec<Srgb> {
        let hues = self.config.hue_strategies.generate_hues();
        let colours = self.config.lum_strategy.attach_lums(&hues);
        let mut colours = self.config.chroma_strategy.attach_chroma(&colours);

        if self.config.misc_flag.extremes {
            colours.push((0.0, 0.0, 0.0));
            colours.push((100.0, 0.0, 0.0));
        }

        colours
            .into_iter()
            .map(|(l, c, h)| Lch::new(l, c, h).into_color())
            .collect()
    }
}
