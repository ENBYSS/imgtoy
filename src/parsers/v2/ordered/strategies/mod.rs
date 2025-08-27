use serde_yaml::Value;

use crate::parsers::v2::{
    ordered::strategies::{
        common::{parse_dimensions_as_f64, parse_matrix_size},
        diagonal::Diagonal,
        increase::Increase,
        orientation::Orientation,
        wrapping::Wrapping,
    },
    structure::value::{
        parse_property_as_f64, parse_property_as_usize, ValueProperty, Vf64, Vusize,
    },
};

pub mod common;
pub mod diagonal;
pub mod increase;
pub mod orientation;
pub mod wrapping;

pub enum Effect {
    Bayer {
        matrix_size: Vusize,
    },
    Diamond {
        matrix_size: Vusize,
    },
    CheckeredDiamonds {
        matrix_size: Vusize,
    },
    Stars,
    NewStars,
    Grid,
    Trail,
    Crisscross,
    Static,
    Wavy {
        orientation: Orientation,
    },
    BootlegBayer,
    Diagonals,
    DiagonalsBig,
    DiamondGrid,
    SpeckleSquares,
    Scales,
    TrailScales,
    DiagonalsN {
        matrix_size: Vusize,
        direction: Diagonal,
        increase: Increase,
    },
    DiagonalTiles {
        matrix_size: Vusize,
    },
    BouncingBowtie {
        matrix_size: Vusize,
    },
    Scanline {
        matrix_size: Vusize,
        orientation: Orientation,
    },
    Starburst {
        matrix_size: Vusize,
    },
    ShinyBowtie {
        matrix_size: Vusize,
    },
    MarbleTile {
        matrix_size: Vusize,
    },
    CurvePath {
        matrix_size: Vusize,
        amplitude: Vf64,
        promotion: Vf64,
        halt_threshold: Vusize,
    },
    Zigzag {
        matrix_size: Vusize,
        halt_threshold: Vusize,
        wrapping: Wrapping,
        magnitude: (Option<Vf64>, Option<Vf64>),
        promotion: (Option<Vf64>, Option<Vf64>),
    },
    BrokenSpiral {
        matrix_size: Vusize,
        base_step: (Option<Vf64>, Option<Vf64>),
        oob_threshold: Vusize,
        increment_by: Vf64,
        increment_in: Vusize,
    },
    ModuloSnake {
        matrix_size: Vusize,
        increment_by: Vf64,
        modulo: Vusize,
        iterations: Vusize,
    },
}

impl Effect {
    pub fn from_value(value: &Value) -> Effect {
        let effect_name = value
            .as_mapping()
            .unwrap()
            .keys()
            .next()
            .unwrap()
            .as_str()
            .unwrap();

        let effect = value.get(effect_name).unwrap();

        match effect_name {
            "bayer" => Self::Bayer {
                matrix_size: parse_matrix_size(effect),
            },
            "diamond" => Self::Diamond {
                matrix_size: parse_matrix_size(effect),
            },
            "checkered-diamonds" => Self::CheckeredDiamonds {
                matrix_size: parse_matrix_size(effect),
            },
            "stars" => Self::Stars,
            "new-stars" => Self::NewStars,
            "grid" => Self::Grid,
            "trail" => Self::Trail,
            "crisscross" => Self::Crisscross,
            "static" => Self::Static,
            "wavy" => Self::Wavy {
                orientation: Orientation::from_value(effect).unwrap_or_default(),
            },
            "bootleg-bayer" => Self::BootlegBayer,
            "diagonals" => Self::Diagonals,
            "diagonals-big" => Self::DiagonalsBig,
            "diagonals-n" => Self::DiagonalsN {
                matrix_size: parse_matrix_size(effect),
                direction: Diagonal::from_value(effect),
                increase: Increase::from_value(effect).unwrap_or_default(),
            },
            "diagonal-tiles" => Self::DiagonalTiles {
                matrix_size: parse_matrix_size(effect),
            },
            "bouncing-bowtie" => Self::BouncingBowtie {
                matrix_size: parse_matrix_size(effect),
            },
            "scanline" => Self::Scanline {
                matrix_size: parse_matrix_size(effect),
                orientation: Orientation::from_value(effect).unwrap_or_default(),
            },
            "starburst" => Self::Starburst {
                matrix_size: parse_matrix_size(effect),
            },
            "shiny-bowtie" => Self::ShinyBowtie {
                matrix_size: parse_matrix_size(effect),
            },
            "marble-tile" => Self::MarbleTile {
                matrix_size: parse_matrix_size(effect),
            },
            "curve-path" => Self::CurvePath {
                matrix_size: parse_matrix_size(effect),
                amplitude: parse_property_as_f64(effect, "amplitude")
                    .unwrap_or(ValueProperty::Fixed(1.0)),
                promotion: parse_property_as_f64(value, "promotion")
                    .unwrap_or(ValueProperty::Fixed(0.0)),
                halt_threshold: parse_property_as_usize(value, "halt-threshold").unwrap(),
            },
            "zigzag" => Self::Zigzag {
                matrix_size: parse_matrix_size(effect),
                halt_threshold: parse_property_as_usize(effect, "halt-threshold").unwrap(),
                wrapping: Wrapping::from_value(effect).unwrap_or_default(),
                magnitude: effect
                    .get("magnitude")
                    .map(|m| parse_dimensions_as_f64(m))
                    .unwrap_or((None, None)),
                promotion: effect
                    .get("promotion")
                    .map(|m| parse_dimensions_as_f64(m))
                    .unwrap_or((None, None)),
            },
            "broken-spiral" => Self::BrokenSpiral {
                matrix_size: parse_matrix_size(effect),
                base_step: effect
                    .get("base-step")
                    .map(|base| parse_dimensions_as_f64(base))
                    .unwrap_or((None, None)),
                oob_threshold: parse_property_as_usize(effect, "oob-threshold")
                    .unwrap_or(ValueProperty::Fixed(100)),
                increment_by: parse_property_as_f64(effect, "increment-by")
                    .unwrap_or(ValueProperty::Fixed(1.0)),
                increment_in: parse_property_as_usize(effect, "increment-in")
                    .unwrap_or(ValueProperty::Fixed(1)),
            },
            "modulo-snake" => Self::ModuloSnake {
                matrix_size: parse_matrix_size(effect),
                increment_by: parse_property_as_f64(effect, "increment-by")
                    .unwrap_or(ValueProperty::Fixed(1.0)),
                modulo: parse_property_as_usize(effect, "modulo")
                    .unwrap_or(ValueProperty::Fixed(10)),
                iterations: parse_property_as_usize(effect, "iterations")
                    .unwrap_or(ValueProperty::Fixed(1)),
            },
            _ => todo!(),
        }
    }
}
