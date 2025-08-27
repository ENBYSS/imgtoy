use image_effects::filter::filters;
use palette::rgb::Rgb;
use serde_yaml::Value;

#[derive(Debug)]
pub struct GradientMap {
    inner: Vec<(Rgb, f32)>,
}

impl GradientMap {
    fn parse_colour(raw: &str) -> Rgb {
        todo!()
    }

    fn parse_colour_threshold(value: &Value) -> (Rgb, f32) {
        let colour = value.get("colour").unwrap().as_str().unwrap();
        let threshold = value.get("threshold").unwrap().as_f64().unwrap();

        todo!()
    }

    pub fn from_value(value: &Value) -> Self {
        todo!("not now, barely used")
    }

    pub fn generate(&self) -> filters::GradientMap {
        todo!("not now, barely used")
    }
}
