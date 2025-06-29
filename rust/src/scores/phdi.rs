use super::{capped_score, DietScore};
use crate::nutrition_vector::NutritionVector;

pub struct PhdiScorer;

impl DietScore for PhdiScorer {
    fn score(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g, 300.0);
        let legumes = capped_score(nv.legumes_g, 100.0);
        let grains = capped_score(nv.whole_grains_g, 90.0);
        let unsat_fat_g = (nv.fat_g - nv.saturated_fat_g - nv.trans_fat_g).max(0.0);
        let unsat_fat = capped_score(unsat_fat_g, 20.0);
        let red_meat = (10.0 - capped_score(nv.red_meat_g, 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar_g, 50.0)).clamp(0.0, 10.0);
        let refined = (10.0 - capped_score(nv.refined_grains_g, 150.0)).clamp(0.0, 10.0);
        let energy = if nv.energy_kcal >= 1500.0 && nv.energy_kcal <= 2500.0 {
            10.0
        } else {
            0.0
        };

        veg + legumes + grains + unsat_fat + red_meat + sugar + refined + energy
    }

    fn name(&self) -> &'static str {
        "PHDI"
    }
}
