use image_effects::{
    dither::{
        self,
        error::{self, WithPalette},
    },
    effect::Effect,
    filter::filters,
};
use serde_yaml::Value;

use crate::parsers::v2::effects::{
    error_propagator::ErrorPropagator,
    gradient_map::GradientMap,
    ordered::Ordered,
    quantize_hue::QuantizeHue,
    simple::{Brighten, Contrast, HueRotate, MultiplyHue, Saturate},
};

pub mod error_propagator;
pub mod gradient_map;
pub mod ordered;
pub mod quantize_hue;
pub mod simple;

#[derive(Debug)]
pub enum EffectKind {
    Brighten(Brighten),
    Saturate(Saturate),
    Contrast(Contrast),
    HueRotate(HueRotate),
    MultiplyHue(MultiplyHue),
    QuantizeHue(QuantizeHue),
    GradientMap(GradientMap),
    ErrorPropagator(ErrorPropagator),
    Ordered(Ordered),
}

impl From<Brighten> for EffectKind {
    fn from(value: Brighten) -> Self {
        Self::Brighten(value)
    }
}

impl From<Saturate> for EffectKind {
    fn from(value: Saturate) -> Self {
        Self::Saturate(value)
    }
}

impl From<Contrast> for EffectKind {
    fn from(value: Contrast) -> Self {
        Self::Contrast(value)
    }
}

impl From<HueRotate> for EffectKind {
    fn from(value: HueRotate) -> Self {
        Self::HueRotate(value)
    }
}

impl From<MultiplyHue> for EffectKind {
    fn from(value: MultiplyHue) -> Self {
        Self::MultiplyHue(value)
    }
}

impl From<QuantizeHue> for EffectKind {
    fn from(value: QuantizeHue) -> Self {
        Self::QuantizeHue(value)
    }
}

impl From<GradientMap> for EffectKind {
    fn from(value: GradientMap) -> Self {
        Self::GradientMap(value)
    }
}

impl From<ErrorPropagator> for EffectKind {
    fn from(value: ErrorPropagator) -> Self {
        Self::ErrorPropagator(value)
    }
}

impl From<Ordered> for EffectKind {
    fn from(value: Ordered) -> Self {
        Self::Ordered(value)
    }
}

impl EffectKind {
    pub fn from_value(value: &Value) -> Self {
        let effect = value
            .as_mapping()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .as_str()
            .unwrap();

        match effect {
            "brighten" => Brighten::from_value(value).into(),
            "saturate" => Saturate::from_value(value).into(),
            "contrast" => Contrast::from_value(value).into(),
            "hue-rotate" => HueRotate::from_value(value).into(),
            "multiply-hue" => MultiplyHue::from_value(value).into(),
            "quantize-hue" => QuantizeHue::from_value(value).into(),
            "gradient-map" => GradientMap::from_value(value).into(),
            "error-propagator" => ErrorPropagator::from_value(value).into(),
            "ordered" => Ordered::from_value(value).into(),
            _ => todo!(),
        }
    }

    pub fn generate<'a, 'b, T>(&self) -> Box<dyn Effect<T>>
    where
        filters::HueRotate: Effect<T>,
        filters::Contrast: Effect<T>,
        filters::Brighten: Effect<T>,
        filters::Saturate: Effect<T>,
        filters::GradientMap: Effect<T>,
        filters::QuantizeHue: Effect<T>,
        filters::MultiplyHue: Effect<T>,
        dither::ordered::Ordered: Effect<T>,
        error::ErrorPropagator<'static, 'static, WithPalette>: Effect<T>,
    {
        match self {
            Self::Brighten(f) => Box::new(f.generate()),
            Self::Saturate(f) => Box::new(f.generate()),
            Self::Contrast(f) => Box::new(f.generate()),
            Self::HueRotate(f) => Box::new(f.generate()),
            Self::MultiplyHue(f) => Box::new(f.generate()),
            Self::QuantizeHue(f) => Box::new(f.generate()),
            Self::GradientMap(f) => Box::new(f.generate()),
            Self::ErrorPropagator(f) => Box::new(f.generate()),
            Self::Ordered(f) => Box::new(f.generate_effect()),
        }
    }
}

#[derive(Debug)]
pub struct Effects {
    kinds: Vec<EffectKind>,
}

impl Effects {
    pub fn from_value(value: &Value) -> Self {
        Self {
            kinds: value
                .get("effects")
                .unwrap()
                .as_sequence()
                .unwrap()
                .iter()
                .map(|s| EffectKind::from_value(s))
                .collect(),
        }
    }

    pub fn generate<T>(&self) -> Vec<Box<dyn Effect<T>>>
    where
        filters::HueRotate: Effect<T>,
        filters::Contrast: Effect<T>,
        filters::Brighten: Effect<T>,
        filters::Saturate: Effect<T>,
        filters::GradientMap: Effect<T>,
        filters::QuantizeHue: Effect<T>,
        filters::MultiplyHue: Effect<T>,
        dither::ordered::Ordered: Effect<T>,
        error::ErrorPropagator<'static, 'static, WithPalette>: Effect<T>,
    {
        self.kinds.iter().map(|kind| kind.generate()).collect()
    }
}
