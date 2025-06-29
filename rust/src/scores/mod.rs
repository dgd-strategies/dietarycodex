use crate::nutrition_vector::NutritionVector;

pub trait DietScore {
    fn score(&self, nv: &NutritionVector) -> f64;
    fn name(&self) -> &'static str;
}

pub fn capped_score(value: f64, max: f64) -> f64 {
    (value / max * 10.0).clamp(0.0, 10.0)
}

pub mod ahei;
pub mod amed;
pub mod dash;
pub mod hei;
pub mod registry;

pub use registry::all_scorers;
