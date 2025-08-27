use std::collections::HashMap;

use serde_yaml::Value;

#[derive(PartialEq, Eq, Hash)]
pub enum DiagonalDirection {
    DownRight,
    UpRight,
}

pub enum DiagonalKind {
    Ratios(HashMap<DiagonalDirection, f64>),
    Exact(DiagonalDirection),
}

pub struct Diagonal {
    kind: DiagonalKind,
}

impl Diagonal {
    pub fn from_value(value: &Value) -> Self {
        let diagonaldirection = value
            .get("diagonal-direction")
            .unwrap_or_else(|| panic!("[ordered.strategy] requires [ordered.diagonal-direction]"));

        match diagonaldirection {
            Value::Mapping(mapping) => {
                let dr_ratio = mapping.get("down-right").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.horizontal] must be a float.".to_string()) }));
                let ur_ratio = mapping.get("up-right").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.vertical] must be a float.".to_string()) }));

                let mut ratio_map = HashMap::new();
                if dr_ratio.is_some() {
                    ratio_map.insert(DiagonalDirection::DownRight, dr_ratio.unwrap());
                }
                if ur_ratio.is_some() {
                    ratio_map.insert(DiagonalDirection::UpRight, ur_ratio.unwrap());
                }

                Diagonal { kind: DiagonalKind::Ratios(ratio_map) }
            },
            Value::String(orientation) => {
                let direction = match orientation.as_str() {
                    "down-right" => DiagonalDirection::DownRight,
                    "up-right" => DiagonalDirection::UpRight,
                    _ => panic!("[ordered.orientation] must be 'down-right', 'up-right', or a mapping of ratios.")
                };

                Diagonal { kind: DiagonalKind::Exact(direction) }
            },
            _ => panic!("[ordered.orientation] must be a mapping of ratios, or one of 'down-right' / 'up-right'")
        }
    }
}
