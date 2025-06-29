use super::{capped_score, DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct PhdiScorer;

impl FieldDeps for PhdiScorer {
    fn name() -> &'static str {
        "PHDI"
    }

    fn required_fields() -> &'static [&'static str] {
        &[
            "vegetables_g",
            "legumes_g",
            "whole_grains_g",
            "fat_g",
            "saturated_fat_g",
            "trans_fat_g",
            "red_meat_g",
            "sugar_g",
            "refined_grains_g",
            "energy_kcal",
        ]
    }
}

impl DietScore for PhdiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g.unwrap_or(0.0), 300.0);
        let legumes = capped_score(nv.legumes_g.unwrap_or(0.0), 100.0);
        let grains = capped_score(nv.whole_grains_g.unwrap_or(0.0), 90.0);
        let unsat_fat_g = (nv.fat_g.unwrap_or(0.0) - nv.saturated_fat_g.unwrap_or(0.0) - nv.trans_fat_g.unwrap_or(0.0)).max(0.0);
        let unsat_fat = capped_score(unsat_fat_g, 20.0);
        let red_meat = (10.0 - capped_score(nv.red_meat_g.unwrap_or(0.0), 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar_g.unwrap_or(0.0), 50.0)).clamp(0.0, 10.0);
        let refined = (10.0 - capped_score(nv.refined_grains_g.unwrap_or(0.0), 150.0)).clamp(0.0, 10.0);
        let energy = if let Some(kcal) = nv.energy_kcal {
            if kcal >= 1500.0 && kcal <= 2500.0 {
                10.0
            } else {
                0.0
            }
        } else {
            10.0
        };

        veg + legumes + grains + unsat_fat + red_meat + sugar + refined + energy
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
