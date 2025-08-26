use serde_yaml::Value;

use crate::parsers::v2::structure::ValueProperty;

pub fn parse_matrix_size(value: &Value) -> ValueProperty<usize> {
    ValueProperty::<usize>::property(value.get("matrix_size").expect("???"))
}

pub fn parse_factor(value: &Value) -> ValueProperty<f64> {
    ValueProperty::<f64>::property(value.get("factor").expect("???"))
}
