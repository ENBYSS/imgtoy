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
            _ => todo!(),
        };

        ChromaStrategy { kind }
    }
}
