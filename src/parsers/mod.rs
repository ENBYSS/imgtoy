pub mod effects;
pub mod error_propagator;
pub mod modifiers;
pub mod ordered;
pub mod palette;
pub mod properties;
pub mod system;
pub mod util;

/// A plan for an upgraded parsing system.
///
/// The main benefit here is that the entire configuration is parsed into
/// a single struct that then gets executed.
///
/// This splits the random generation away from the parsing - since currently
/// we have to reparse the file EVERY TIME we iterate.
pub mod v2;
