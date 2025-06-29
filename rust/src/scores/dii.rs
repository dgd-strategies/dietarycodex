use super::DietScore;
use crate::nutrition_vector::NutritionVector;

pub struct DiiScorer;

impl DietScore for DiiScorer {
    fn score(&self, nv: &NutritionVector) -> f64 {
        // Placeholder weighted scoring reflecting inflammatory potential.
        let mut score = 0.0;
        // pro-inflammatory components
        score += nv.saturated_fat_g * 0.1;
        score += nv.trans_fat_g * 0.3;
        score += nv.sugar_g * 0.05;
        // anti-inflammatory components
        score -= nv.fiber_g * 0.1;
        score -= nv.vitamin_c_mg * 0.005;
        score -= nv.vitamin_a_mcg * 0.0001;
        score -= nv.vitamin_e_mg * 0.02;
        score -= nv.omega3_g * 0.3;
        score -= nv.zinc_mg * 0.05;
        score -= nv.selenium_mcg * 0.003;
        score -= nv.magnesium_mg * 0.01;
        score
    }

    fn name(&self) -> &'static str {
        "DII"
    }
}
