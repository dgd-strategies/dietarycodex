use super::{capped_score, DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct AMedScorer;

impl FieldDeps for AMedScorer {
    fn name() -> &'static str {
        "aMED"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("aMED")
    }
}

impl DietScore for AMedScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g.unwrap_or(0.0), 300.0);
        let legumes = capped_score(nv.legumes_g.unwrap_or(0.0), 100.0);
        let fruit = capped_score(nv.total_fruits_g.unwrap_or(0.0), 200.0);
        let grains = capped_score(nv.whole_grains_g.unwrap_or(0.0), 75.0);
        let fish = capped_score(nv.fish_g.unwrap_or(0.0), 100.0);
        let mono_fat = capped_score(nv.mono_fat_g.unwrap_or(0.0), 25.0);
        let red_meat = (10.0 - capped_score(nv.red_meat_g.unwrap_or(0.0), 100.0)).clamp(0.0, 10.0);
        // Placeholder: alcohol component omitted for now
        veg + legumes + fruit + grains + fish + mono_fat + red_meat
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
