use super::{capped_score, DietScore};
use crate::nutrition_vector::NutritionVector;

pub struct DashiScorer;

impl DietScore for DashiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g, 400.0) / 5.0;
        let fruit = capped_score(nv.total_fruits_g, 400.0) / 5.0;
        let dairy = capped_score(nv.calcium_mg, 1000.0) / 5.0;
        let grains = capped_score(nv.whole_grains_g, 75.0) / 5.0;
        let sodium = if nv.sodium_mg <= 1500.0 {
            2.0
        } else if nv.sodium_mg >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium_mg) / (2300.0 - 1500.0) * 2.0
        };
        veg + fruit + dairy + grains + sodium
    }

    fn name(&self) -> &'static str {
        "DASHI"
    }
}
