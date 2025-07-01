//! Available scorers: AHEI, HEI, DASH, aMED, DII, ACS2020, PHDI, DASHI, MIND.
//!
//! Partial evaluation is supported through [`evaluate_allow_partial`](crate::eval::evaluate_allow_partial).
//! Each scorer returns a [`ScoreInfo`](crate::eval::ScoreInfo) indicating
//! the computed value and validation status. When running the CLI with
//! `--allow-partial --verbose-partial`, skipped scores are grouped and printed
//! in alphabetical order before the JSON result so missing fields are easy to
//! spot.
//! Helper functions like [`format_skipped_scores`](crate::eval::format_skipped_scores)
//! can be used to present this information outside of the CLI.

use crate::nutrition_vector::NutritionVector;

pub trait FieldDeps {
    fn name() -> &'static str;
    fn required_fields() -> &'static [&'static str];
}

pub trait DietScore {
    fn name(&self) -> &'static str;
    fn evaluate(&self, nv: &NutritionVector) -> f64;
    fn required_fields(&self) -> &'static [&'static str];
}

pub fn capped_score(value: f64, max: f64) -> f64 {
    (value / max * 10.0).clamp(0.0, 10.0)
}

pub mod acs2020;
pub mod ahei;
pub mod amed;
pub mod dash;
pub mod dashi;
pub mod dii;
pub mod hei;
pub mod mind;
pub mod phdi;
pub mod registry;

/// Available scorers: AHEI, HEI, DASH, aMED, DII, ACS2020, PHDI, DASHI, MIND
pub use registry::all_scorers;
