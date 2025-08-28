use image_effects::dither::ordered::algorithms::properties;
use rand::Rng;
use serde_yaml::Value;

#[derive(PartialEq, Eq, Hash, Debug)]
pub enum DiagonalDirection {
    DownRight,
    UpRight,
}

impl From<&DiagonalDirection> for properties::DiagonalDirection {
    fn from(value: &DiagonalDirection) -> Self {
        match value {
            DiagonalDirection::DownRight => properties::DiagonalDirection::DownRight,
            DiagonalDirection::UpRight => properties::DiagonalDirection::UpRight,
        }
    }
}

#[derive(Debug)]
pub enum DiagonalKind {
    Ratios(Vec<(f64, DiagonalDirection)>),
    Exact(DiagonalDirection),
}

#[derive(Debug)]
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

                let mut ratio_map = Vec::new();
                if dr_ratio.is_some() {
                    ratio_map.push((dr_ratio.unwrap(), DiagonalDirection::DownRight));
                }
                if ur_ratio.is_some() {
                    ratio_map.push((ur_ratio.unwrap(), DiagonalDirection::UpRight));
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

    pub fn generate(&self) -> properties::DiagonalDirection {
        match &self.kind {
            DiagonalKind::Exact(direction) => direction.into(),
            DiagonalKind::Ratios(ratios) => {
                let capacity = ratios.iter().map(|(ratio, _)| ratio).sum();

                let mut flag = rand::rng().random_range(0.0..capacity);

                for (ratio, direction) in ratios {
                    flag -= ratio;
                    if flag <= 0.0 {
                        return direction.into();
                    }
                }

                todo!("fix ratio calculation")
            }
        }
    }
}
