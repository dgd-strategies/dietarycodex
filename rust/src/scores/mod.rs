//! Available scores: AHEI, HEI, DASH, DASHI, aMED, DII, ACS2020, PHDI, MIND

use crate::nutrition_vector::NutritionVector;

pub trait DietScore {
    fn score(&self, nv: &NutritionVector) -> f64;
    fn name(&self) -> &'static str;
}

pub fn capped_score(value: f64, max: f64) -> f64 {
    (value / max * 10.0).clamp(0.0, 10.0)
}

pub fn format_score_name<T: DietScore>(scorer: &T) -> String {
    scorer.name().to_string()
}

pub mod ahei;
pub mod amed;
pub mod dash;
pub mod dashi;
pub mod dii;
pub mod hei;
pub mod acs2020;
pub mod phdi;
pub mod mind;
pub mod registry;

pub use registry::all_scorers;
