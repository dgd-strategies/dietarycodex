use super::{capped_score, DietScore};
use crate::nutrition_vector::NutritionVector;

pub struct MindScorer;

impl DietScore for MindScorer {
    fn score(&self, nv: &NutritionVector) -> f64 {
        let leafy = capped_score(nv.vegetables_g, 100.0) / 10.0;
        let berries = capped_score(nv.berries_g, 50.0) / 10.0;
        let nuts = capped_score(nv.nuts_g, 30.0) / 10.0;
        let grains = capped_score(nv.whole_grains_g, 60.0) / 10.0;
        let fish = capped_score(nv.fish_g, 100.0) / 10.0;
        let poultry = capped_score(nv.poultry_g, 100.0) / 10.0;
        let olive_oil = capped_score(nv.mono_fat_g, 20.0) / 10.0;

        let red_meat = (1.0 - capped_score(nv.red_meat_g, 100.0) / 10.0).clamp(0.0, 1.0);
        let fast_food = (1.0 - capped_score(nv.fast_food_g, 100.0) / 10.0).clamp(0.0, 1.0);
        let sweets = (1.0 - capped_score(nv.sugar_g, 50.0) / 10.0).clamp(0.0, 1.0);
        let cheese = (1.0 - capped_score(nv.cheese_g, 50.0) / 10.0).clamp(0.0, 1.0);
        let butter = (1.0 - capped_score(nv.butter_g, 20.0) / 10.0).clamp(0.0, 1.0);

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
        "MIND"
    }
}
