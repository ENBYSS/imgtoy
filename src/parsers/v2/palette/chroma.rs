use rand::Rng;
use serde_yaml::Value;

#[derive(Debug)]
pub enum ChromaStrategyKind {
    Random,
}

#[derive(Debug)]
pub struct ChromaStrategy {
    kind: ChromaStrategyKind,
}

impl ChromaStrategy {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "random" => ChromaStrategyKind::Random,
            _ => unimplemented!("strategy {kind} is not supported."),
        };

        ChromaStrategy { kind }
    }

    pub fn attach_chroma(&self, colours: &Vec<(f32, f32)>) -> Vec<(f32, f32, f32)> {
        let mut rng = rand::rng();
        colours
            .iter()
            .map(|(hue, lum)| (*hue, rng.random_range(0.0..128.0), *lum))
            .collect()
    }
}
