//! Available scorers: AHEI, HEI, DASH, aMED, DII, ACS2020, PHDI, DASHI, MIND

use crate::nutrition_vector::NutritionVector;

pub trait DietScore {
    fn name(&self) -> &'static str;
    fn evaluate(&self, nv: &NutritionVector) -> f64;
}

pub fn capped_score(value: f64, max: f64) -> f64 {
    (value / max * 10.0).clamp(0.0, 10.0)
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

/// Available scorers: AHEI, HEI, DASH, aMED, DII, ACS2020, PHDI, DASHI, MIND
pub use registry::all_scorers;
