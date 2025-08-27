use rand::{seq::IndexedRandom, Rng};
use serde_yaml::Value;

trait _Value {}

impl _Value for usize {}
impl _Value for isize {}
impl _Value for f64 {}

#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub enum ValueProperty<T: _Value> {
    Fixed(T),
    Choice(Vec<T>),
    Range(T, T),
}

pub type Vf64 = ValueProperty<f64>;
pub type Vusize = ValueProperty<usize>;
pub type Visize = ValueProperty<isize>;

impl<T: _Value> From<T> for ValueProperty<T> {
    fn from(value: T) -> Self {
        Self::Fixed(value)
    }
}

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

    pub fn generate(&self) -> usize {
        match self {
            ValueProperty::Fixed(val) => *val,
            ValueProperty::Choice(vals) => *vals.choose(&mut rand::rng()).unwrap(),
            ValueProperty::Range(min, max) => rand::rng().random_range(*min..*max),
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

    pub fn generate(&self) -> isize {
        match self {
            ValueProperty::Fixed(val) => *val,
            ValueProperty::Choice(vals) => *vals.choose(&mut rand::rng()).unwrap(),
            ValueProperty::Range(min, max) => todo!("currently can't form a range over isize"),
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

    pub fn generate(&self) -> f64 {
        match self {
            ValueProperty::Fixed(val) => *val,
            ValueProperty::Choice(vals) => *vals.choose(&mut rand::rng()).unwrap(),
            ValueProperty::Range(min, max) => rand::rng().random_range(*min..*max),
        }
    }
}

#[inline]
pub fn parse_property_as_usize(value: &Value, name: &str) -> Option<ValueProperty<usize>> {
    value.get(name).map(|v| ValueProperty::<usize>::property(v))
}

#[inline]
pub fn parse_property_as_isize(value: &Value, name: &str) -> Option<ValueProperty<isize>> {
    value.get(name).map(|v| ValueProperty::<isize>::property(v))
}

#[inline]
pub fn parse_property_as_f64(value: &Value, name: &str) -> Option<ValueProperty<f64>> {
    value.get(name).map(|v| ValueProperty::<f64>::property(v))
}

#[derive(Debug, Clone)]
pub struct Chance {
    value: ValueProperty<f64>,
}

impl From<ValueProperty<f64>> for Chance {
    fn from(value: ValueProperty<f64>) -> Self {
        Chance { value }
    }
}

impl Chance {
    pub fn roll(&self) -> bool {
        let roll = rand::rng().random_range(0.0..=1.0);
        self.value.generate() < roll
    }
}
