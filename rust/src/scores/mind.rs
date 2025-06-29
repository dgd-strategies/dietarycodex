use super::{capped_score, DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct MindScorer;

impl FieldDeps for MindScorer {
    fn name() -> &'static str {
        "MIND"
    }

    fn required_fields() -> &'static [&'static str] {
        &[
            "berries_g",
            "butter_g",
            "cheese_g",
            "fast_food_g",
            "fish_g",
            "mono_fat_g",
            "nuts_g",
            "poultry_g",
            "red_meat_g",
            "sugar_g",
            "vegetables_g",
            "whole_grains_g",
        ]
    }
}

impl DietScore for MindScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let leafy = capped_score(nv.vegetables_g.unwrap_or(0.0), 100.0) / 10.0;
        let berries = capped_score(nv.berries_g.unwrap_or(0.0), 50.0) / 10.0;
        let nuts = capped_score(nv.nuts_g.unwrap_or(0.0), 30.0) / 10.0;
        let grains = capped_score(nv.whole_grains_g.unwrap_or(0.0), 60.0) / 10.0;
        let fish = capped_score(nv.fish_g.unwrap_or(0.0), 100.0) / 10.0;
        let poultry = capped_score(nv.poultry_g.unwrap_or(0.0), 100.0) / 10.0;
        let olive_oil = capped_score(nv.mono_fat_g.unwrap_or(0.0), 20.0) / 10.0;

        let red_meat =
            (1.0 - capped_score(nv.red_meat_g.unwrap_or(0.0), 100.0) / 10.0).clamp(0.0, 1.0);
        let fast_food =
            (1.0 - capped_score(nv.fast_food_g.unwrap_or(0.0), 100.0) / 10.0).clamp(0.0, 1.0);
        let sweets = (1.0 - capped_score(nv.sugar_g.unwrap_or(0.0), 50.0) / 10.0).clamp(0.0, 1.0);
        let cheese = (1.0 - capped_score(nv.cheese_g.unwrap_or(0.0), 50.0) / 10.0).clamp(0.0, 1.0);
        let butter = (1.0 - capped_score(nv.butter_g.unwrap_or(0.0), 20.0) / 10.0).clamp(0.0, 1.0);

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
