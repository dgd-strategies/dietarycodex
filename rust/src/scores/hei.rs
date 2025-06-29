use super::{DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct HeiScorer;

impl FieldDeps for HeiScorer {
    fn name() -> &'static str {
        "HEI"
    }

    fn required_fields() -> &'static [&'static str] {
        &["sodium_mg", "total_fruits_g", "whole_grains_g"]
    }
}

impl DietScore for HeiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let mut score = 0.0;
        // fruit component: 0-10 points with 200g threshold
        score += (nv.total_fruits_g.unwrap_or(0.0) / 200.0 * 10.0).clamp(0.0, 10.0);
        // whole grains component: 0-10 points with 75g threshold
        score += (nv.whole_grains_g.unwrap_or(0.0) / 75.0 * 10.0).clamp(0.0, 10.0);
        // sodium moderation: linear 10 points at <=1500 mg down to 0 at 2300 mg
        let sodium_score = if nv.sodium_mg.unwrap_or(f64::INFINITY) <= 1500.0 {
            10.0
        } else if nv.sodium_mg.unwrap_or(0.0) >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium_mg.unwrap_or(0.0)) / (2300.0 - 1500.0) * 10.0
        };
        score + sodium_score
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
