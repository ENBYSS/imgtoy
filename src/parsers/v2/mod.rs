use serde_yaml::Value;

pub mod effects;
pub mod ordered;
pub mod structure;

pub fn parse_property_as_str(value: &Value, name: &str) -> String {
    value.get(name).unwrap().as_str().unwrap().to_string()
}
