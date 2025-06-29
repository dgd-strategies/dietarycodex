use crate::nutrition_vector::NutritionVector;

pub trait DietScore {
    fn score(&self, nv: &NutritionVector) -> f64;
    fn name(&self) -> &'static str;
}

pub mod ahei;
pub mod hei;
pub mod registry;

pub use registry::all_scorers;
