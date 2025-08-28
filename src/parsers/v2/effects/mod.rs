use image_effects::{
    dispatch::{self, EffectEnum},
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
            _ => unimplemented!("effect {effect} is not supported."),
        }
    }

    pub fn generate<'a, 'b, T>(&self) -> EffectEnum<T>
    where
        dispatch::Null<T>: Effect<T>,
        filters::Invert: Effect<T>,
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
            Self::Brighten(f) => f.generate().into(),
            Self::Saturate(f) => f.generate().into(),
            Self::Contrast(f) => f.generate().into(),
            Self::HueRotate(f) => f.generate().into(),
            Self::MultiplyHue(f) => f.generate().into(),
            Self::QuantizeHue(f) => f.generate().into(),
            Self::GradientMap(f) => f.generate().into(),
            Self::ErrorPropagator(f) => f.generate().into(),
            Self::Ordered(f) => f.generate_effect().into(),
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

    pub fn generate<T>(&self) -> Vec<EffectEnum<T>>
    where
        dispatch::Null<T>: Effect<T>,
        filters::Invert: Effect<T>,
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

// trait EffectDetails {
//     fn details(&self) -> String;
// }

// impl<T> EffectDetails for EffectEnum<T>
// where
//     dispatch::Null<T>: Effect<T>,
//     filters::Invert: Effect<T>,
//     filters::HueRotate: Effect<T>,
//     filters::Contrast: Effect<T>,
//     filters::Brighten: Effect<T>,
//     filters::Saturate: Effect<T>,
//     filters::GradientMap: Effect<T>,
//     filters::QuantizeHue: Effect<T>,
//     filters::MultiplyHue: Effect<T>,
//     dither::ordered::Ordered: Effect<T>,
//     error::ErrorPropagator<'static, 'static, WithPalette>: Effect<T>,
// {
//     fn details(&self) -> String {
//         match self {
//             Self::HueRotate(fx) => format!("hue-rotate ({}deg)", fx.0),
//             Self::Contrast(fx) => format!("contrast ({})", fx.0),
//             Self::Brighten(fx) => format!("brighten ({})", fx.0),
//             Self::Saturate(fx) => format!("saturate ({})", fx.0),
//             Self::GradientMap(fx) => format!("gradient-map (!!!)"),
//             Self::QuantizeHue(fx) => format!("quantize-hue (!!!)"),
//             Self::MultiplyHue(fx) => format!("multiply-hue (!!!)"),
//             Self::Ordered(fx) => format!("ordered (!!!)"),
//             Self::ErrorPropagator(fx) => format!("error-propagator (!!!)"),
//             Self::Null(fx) => format!("null (???)"),
//             Self::Invert(fx) => format!("invert"),
//         }
//     }
// }
