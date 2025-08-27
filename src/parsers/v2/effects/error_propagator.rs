use image_effects::dither::{
    error::{self, Base, WithPalette},
    ATKINSON, BURKES, FLOYD_STEINBERG, JARVIS_JUDICE_NINKE, SIERRA, SIERRA_LITE, SIERRA_TWO_ROW,
    STUCKI,
};
use serde_yaml::Value;

use crate::parsers::v2::palette::Palette;

#[derive(Debug)]
pub enum ErrorPropagatorKind {
    FloydSteinberg,
    JarvisJudiceNinke,
    Atkinson,
    Burkes,
    Stucki,
    Sierra,
    SierraTwoRow,
    SierraLite,
}

impl ErrorPropagatorKind {
    pub fn get(&self) -> error::ErrorPropagator<'static, 'static, Base> {
        match self {
            Self::FloydSteinberg => FLOYD_STEINBERG,
            Self::JarvisJudiceNinke => JARVIS_JUDICE_NINKE,
            Self::Atkinson => ATKINSON,
            Self::Burkes => BURKES,
            Self::Stucki => STUCKI,
            Self::Sierra => SIERRA,
            Self::SierraTwoRow => SIERRA_TWO_ROW,
            Self::SierraLite => SIERRA_LITE,
        }
    }
}

#[derive(Debug)]
pub struct ErrorPropagator {
    kind: ErrorPropagatorKind,
    palette: Palette,
}

impl ErrorPropagator {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "floydsteinberg" | "floyd-steinberg" | "floyd_steinberg" => {
                ErrorPropagatorKind::FloydSteinberg
            }
            "jarvisjudiceninke" | "jarvis-judice-ninke" | "jarvis_judice_ninke" => {
                ErrorPropagatorKind::JarvisJudiceNinke
            }
            "atkinson" => ErrorPropagatorKind::Atkinson,
            "burkes" => ErrorPropagatorKind::Burkes,
            "stucki" => ErrorPropagatorKind::Stucki,
            "sierra" => ErrorPropagatorKind::Sierra,
            "sierra-two-row" | "sierra_two_row" => ErrorPropagatorKind::SierraTwoRow,
            "sierra-lite" | "sierra_to_row" => ErrorPropagatorKind::SierraLite,
            _ => todo!(),
        };

        let palette = Palette::from_value(value);

        ErrorPropagator { kind, palette }
    }

    pub fn generate(&self) -> error::ErrorPropagator<'static, 'static, WithPalette> {
        let kind = self.kind.get();
        let kind = kind.with_palette(self.palette.generate());
        return kind;
    }
}
