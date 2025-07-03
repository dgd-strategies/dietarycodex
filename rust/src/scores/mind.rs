use super::{capped_score, DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct MindScorer;

impl FieldDeps for MindScorer {
    fn name() -> &'static str {
        "MIND"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("MIND")
    }
}

impl DietScore for MindScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let leafy = capped_score(nv.vegetables.unwrap_or(0.0), 100.0) / 10.0;
        let berries = capped_score(nv.berries.unwrap_or(0.0), 50.0) / 10.0;
        let nuts = capped_score(nv.nuts.unwrap_or(0.0), 30.0) / 10.0;
        let grains = capped_score(nv.whole_grains.unwrap_or(0.0), 60.0) / 10.0;
        let fish = capped_score(nv.fish.unwrap_or(0.0), 100.0) / 10.0;
        let poultry = capped_score(nv.poultry.unwrap_or(0.0), 100.0) / 10.0;
        let olive_oil = capped_score(nv.mono_fat.unwrap_or(0.0), 20.0) / 10.0;

        let red_meat =
            (1.0 - capped_score(nv.red_meat.unwrap_or(0.0), 100.0) / 10.0).clamp(0.0, 1.0);
        let fast_food =
            (1.0 - capped_score(nv.fast_food.unwrap_or(0.0), 100.0) / 10.0).clamp(0.0, 1.0);
        let sweets = (1.0 - capped_score(nv.sugar.unwrap_or(0.0), 50.0) / 10.0).clamp(0.0, 1.0);
        let cheese = (1.0 - capped_score(nv.cheese.unwrap_or(0.0), 50.0) / 10.0).clamp(0.0, 1.0);
        let butter = (1.0 - capped_score(nv.butter.unwrap_or(0.0), 20.0) / 10.0).clamp(0.0, 1.0);

        leafy
            + berries
            + nuts
            + grains
            + fish
            + poultry
            + olive_oil
            + red_meat
            + fast_food
            + sweets
            + cheese
            + butter
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
