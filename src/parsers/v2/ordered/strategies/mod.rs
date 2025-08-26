use crate::parsers::v2::structure::{Vf64, Vusize};

pub mod common;

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
        orientation: String,
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
        direction: String,
        increase: String,
    },
    DiagonalTiles {
        matrix_size: Vusize,
    },
    BouncingBowtie {
        matrix_size: Vusize,
    },
    Scanline {
        matrix_size: Vusize,
        orientation: String,
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
        wrapping: String,
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
