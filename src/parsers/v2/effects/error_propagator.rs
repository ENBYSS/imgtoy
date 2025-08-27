use serde_yaml::Value;

pub enum ErrorPropagatorKind {
    FloydSteinberg,
    JarvisJudiceNinke,
    Atkinson,
    Burkes,
    Stucki,
    Sierra,
    SierraTwoRow,
    SierraLite,
}

pub struct ErrorPropagator {
    kind: ErrorPropagatorKind,
}

impl ErrorPropagator {
    pub fn from_value(value: &Value) -> Self {
        let kind = value.get("type").unwrap().as_str().unwrap();

        let kind = match kind {
            "floydsteinberg" | "floyd-steinberg" | "floyd_steinberg" => {
                ErrorPropagatorKind::FloydSteinberg
            }
            "jarvisjudiceninke" | "jarvis-judice-ninke" | "jarvis_judice_ninke" => {
                ErrorPropagatorKind::JarvisJudiceNinke
            }
            "atkinson" => ErrorPropagatorKind::Atkinson,
            "burkes" => ErrorPropagatorKind::Burkes,
            "stucki" => ErrorPropagatorKind::Stucki,
            "sierra" => ErrorPropagatorKind::Sierra,
            "sierra-two-row" | "sierra_two_row" => ErrorPropagatorKind::SierraTwoRow,
            "sierra-lite" | "sierra_to_row" => ErrorPropagatorKind::SierraLite,
            _ => todo!(),
        };

        ErrorPropagator { kind }
    }
}
