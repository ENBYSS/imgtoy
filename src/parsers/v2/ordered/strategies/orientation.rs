use std::collections::HashMap;

use serde_yaml::Value;

#[derive(PartialEq, Eq, Hash)]
pub enum OrientationValueKind {
    Horizontal,
    Vertical,
}

pub enum OrientationKind {
    Exact(OrientationValueKind),
    Ratios(HashMap<OrientationValueKind, f64>),
}

pub struct Orientation {
    kind: OrientationKind,
}

impl Default for Orientation {
    fn default() -> Self {
        Orientation {
            kind: OrientationKind::Exact(OrientationValueKind::Horizontal),
        }
    }
}

impl Orientation {
    pub fn from_value(value: &Value) -> Option<Self> {
        let orientation = value.get("orientation");
        if orientation.is_none() {
            return None;
        }
        let orientation = orientation.unwrap();

        let kind = match orientation {
        Value::Mapping(mapping) => {
            let horizontal_ratio = mapping.get("horizontal").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.horizontal] must be a float.".to_string()) }));
            let vertical_ratio = mapping.get("vertical").map(|ratio| ratio.as_f64().unwrap_or_else(|| { panic!("{}", "[ordered.orientation.vertical] must be a float.".to_string()) }));

            let mut ratios = HashMap::new();

            if horizontal_ratio.is_some() {
                ratios.insert(OrientationValueKind::Horizontal, horizontal_ratio.unwrap());
            }

            if vertical_ratio.is_some() {
                ratios.insert(OrientationValueKind::Vertical, vertical_ratio.unwrap());
            }

            OrientationKind::Ratios(ratios)
        },
        Value::String(orientation) => {
            let orientation_value_kind = match orientation.as_str() {
                "horizontal" => OrientationValueKind::Horizontal,
                "vertical" => OrientationValueKind::Vertical,
                _ => panic!("[ordered.orientation] must be 'horizontal', 'vertical', or a mapping of ratios.")
            };

            OrientationKind::Exact(orientation_value_kind)
        },
        _ => panic!("[ordered.orientation] must be a mapping of ratios, or one of 'vertical' / 'horizontal'")
    };

        Some(Orientation { kind })
    }
}
