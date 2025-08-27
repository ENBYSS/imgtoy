use serde_yaml::Value;

#[derive(Debug)]
pub enum WrappingKind {
    Horizontal,
    Vertical,
    All,
    None,
}

#[derive(Debug)]
pub struct Wrapping {
    kinds: Vec<WrappingKind>,
}

impl Default for Wrapping {
    fn default() -> Self {
        Wrapping {
            kinds: vec![WrappingKind::None],
        }
    }
}

impl Wrapping {
    pub fn from_value(value: &Value) -> Option<Self> {
        let wrappings = value.get("wrapping");
        if wrappings.is_none() {
            return None;
        }

        let wrappings = wrappings
            .unwrap()
            .as_sequence()
            .expect("[wrappings] must be a list.");

        let kinds = wrappings
            .iter()
            .map(|wrapping| wrapping.as_str().expect("[wrappings[$]] must be a string."))
            .map(|wrapping| match wrapping {
                "horizontal" => WrappingKind::Horizontal,
                "vertical" => WrappingKind::Vertical,
                "all" => WrappingKind::All,
                "none" => WrappingKind::None,
                _ => todo!(),
            })
            .collect::<Vec<_>>();

        Some(Wrapping { kinds })
    }
}
