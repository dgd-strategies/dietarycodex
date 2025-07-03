use super::{capped_score, DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct PhdiScorer;

impl FieldDeps for PhdiScorer {
    fn name() -> &'static str {
        "PHDI"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("PHDI")
    }
}

impl DietScore for PhdiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables.unwrap_or(0.0), 300.0);
        let legumes = capped_score(nv.legumes.unwrap_or(0.0), 100.0);
        let grains = capped_score(nv.whole_grains.unwrap_or(0.0), 90.0);
        let unsat_fat_g = (nv.fat.unwrap_or(0.0)
            - nv.saturated_fat.unwrap_or(0.0)
            - nv.trans_fat.unwrap_or(0.0))
        .max(0.0);
        let unsat_fat = capped_score(unsat_fat_g, 20.0);
        let red_meat = (10.0 - capped_score(nv.red_meat.unwrap_or(0.0), 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar.unwrap_or(0.0), 50.0)).clamp(0.0, 10.0);
        let refined =
            (10.0 - capped_score(nv.refined_grains.unwrap_or(0.0), 150.0)).clamp(0.0, 10.0);
        let energy = if let Some(kcal) = nv.energy {
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
