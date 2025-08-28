use serde_yaml::Value;

use crate::parsers::v2::structure::value::{
    parse_property_as_f64, parse_property_as_usize, ValueProperty,
};

pub fn parse_matrix_size(value: &Value) -> ValueProperty<usize> {
    parse_property_as_usize(value, "matrix-size").unwrap()
}

pub fn parse_dimensions_as_f64(
    value: &Value,
) -> (Option<ValueProperty<f64>>, Option<ValueProperty<f64>>) {
    (
        parse_property_as_f64(value, "x"),
        parse_property_as_f64(value, "y"),
    )
}
