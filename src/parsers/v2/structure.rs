use serde_yaml::Value;

pub enum SourceKind {
    Url(String),
    File(String),
}

pub struct Source {
    kind: SourceKind,
    max_dim: Option<usize>,
}

pub struct Output {
    path: String,
    n: usize,
}

trait _Value {}

impl _Value for usize {}
impl _Value for isize {}
impl _Value for f64 {}

pub enum ValueProperty<T: _Value> {
    Fixed(T),
    Choice(Vec<T>),
    Range(T, T),
}

pub type Vf64 = ValueProperty<f64>;
pub type Vusize = ValueProperty<usize>;
pub type Visize = ValueProperty<isize>;

impl ValueProperty<usize> {
    pub fn property(value: &Value) -> Self {
        if let Some(exact) = value.as_u64() {
            ValueProperty::Fixed(exact as usize)
        } else if let Some(range) = value.as_mapping() {
            let min = range
                .get("min")
                .expect("expected [brighten.min] due to mapping - not present.")
                .as_u64()
                .expect("[brighten.min] must be a valid float - wasn't.");

            let max = range
                .get("max")
                .expect("expected [brighten.max] due to mapping - not present.")
                .as_u64()
                .expect("[brighten.max] must be a valid float - wasn't.");

            ValueProperty::Range(min as usize, max as usize)
        } else if let Some(options) = value.as_sequence() {
            ValueProperty::Choice(
                options
                    .iter()
                    .map(|val| val.as_u64().unwrap() as usize)
                    .collect(),
            )
        } else {
            todo!()
        }
    }
}

impl ValueProperty<isize> {
    pub fn property(value: &Value) -> Self {
        if let Some(exact) = value.as_i64() {
            ValueProperty::Fixed(exact as isize)
        } else if let Some(range) = value.as_mapping() {
            let min = range
                .get("min")
                .expect("expected [brighten.min] due to mapping - not present.")
                .as_i64()
                .expect("[brighten.min] must be a valid float - wasn't.");

            let max = range
                .get("max")
                .expect("expected [brighten.max] due to mapping - not present.")
                .as_i64()
                .expect("[brighten.max] must be a valid float - wasn't.");

            ValueProperty::Range(min as isize, max as isize)
        } else if let Some(options) = value.as_sequence() {
            ValueProperty::Choice(
                options
                    .iter()
                    .map(|val| val.as_i64().unwrap() as isize)
                    .collect(),
            )
        } else {
            todo!()
        }
    }
}

impl ValueProperty<f64> {
    pub fn property(value: &Value) -> Self {
        if let Some(exact) = value.as_f64() {
            ValueProperty::Fixed(exact as f64)
        } else if let Some(range) = value.as_mapping() {
            let min = range
                .get("min")
                .expect("expected [brighten.min] due to mapping - not present.")
                .as_f64()
                .expect("[brighten.min] must be a valid float - wasn't.");

            let max = range
                .get("max")
                .expect("expected [brighten.max] due to mapping - not present.")
                .as_f64()
                .expect("[brighten.max] must be a valid float - wasn't.");

            ValueProperty::Range(min as f64, max as f64)
        } else if let Some(options) = value.as_sequence() {
            ValueProperty::Choice(
                options
                    .iter()
                    .map(|val| val.as_f64().unwrap() as f64)
                    .collect(),
            )
        } else {
            todo!()
        }
    }
}
