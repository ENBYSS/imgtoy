pub mod gradient_map;

#[derive(Debug)]
pub enum EffectKind {
    HueRotate,
    Contrast,
    Brighten,
    Saturate,
    GradientMap,
    QuantizeHue,
    MultiplyHue,

    Ordered,
    ErrorPropagator,
}

impl From<&str> for EffectKind {
    fn from(value: &str) -> Self {
        match value {
            "hue-rotate" => Self::HueRotate,
            "contrast" => Self::Contrast,
            "brighten" => Self::Brighten,
            "saturate" => Self::Saturate,
            "gradient-map" => Self::GradientMap,
            "quantize-hue" => Self::QuantizeHue,
            "multiply-hue" => Self::MultiplyHue,
            "ordered" => Self::Ordered,
            _ => Self::ErrorPropagator,
        }
    }
}
