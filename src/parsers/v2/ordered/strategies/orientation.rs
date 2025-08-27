use std::collections::HashMap;

use image_effects::dither::ordered::algorithms::properties;
use rand::Rng;
use serde_yaml::Value;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum OrientationValueKind {
    Horizontal,
    Vertical,
}

impl From<&OrientationValueKind> for properties::Orientation {
    fn from(value: &OrientationValueKind) -> Self {
        match value {
            OrientationValueKind::Horizontal => properties::Orientation::Horizontal,
            OrientationValueKind::Vertical => properties::Orientation::Vertical,
        }
    }
}

#[derive(Debug)]
pub enum OrientationKind {
    Exact(OrientationValueKind),
    Ratios(Vec<(f64, OrientationValueKind)>),
}

#[derive(Debug)]
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

            let mut ratios = Vec::new();

            if horizontal_ratio.is_some() {
                ratios.push((horizontal_ratio.unwrap(), OrientationValueKind::Horizontal));
            }

            if vertical_ratio.is_some() {
                ratios.push((vertical_ratio.unwrap(), OrientationValueKind::Vertical,));
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

    pub fn generate(&self) -> properties::Orientation {
        match &self.kind {
            OrientationKind::Exact(orientation) => orientation.into(),
            OrientationKind::Ratios(ratios) => {
                let capacity = ratios.iter().map(|(ratio, _)| ratio).sum();

                let mut flag = rand::rng().random_range(0.0..capacity);

                for (ratio, orientation) in ratios {
                    flag -= ratio;
                    if flag <= 0.0 {
                        return orientation.into();
                    }
                }

                todo!("fix ratio calculation")
            }
        }
    }
}
