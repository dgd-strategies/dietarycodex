use super::{capped_score, DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct DashiScorer;

impl FieldDeps for DashiScorer {
    fn name() -> &'static str {
        "DASHI"
    }

    fn required_fields() -> &'static [&'static str] {
        &[
            "vegetables_g",
            "total_fruits_g",
            "calcium_mg",
            "whole_grains_g",
            "sodium_mg",
        ]
    }
}

impl DietScore for DashiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g.unwrap_or(0.0), 400.0) / 5.0;
        let fruit = capped_score(nv.total_fruits_g.unwrap_or(0.0), 400.0) / 5.0;
        let dairy = capped_score(nv.calcium_mg.unwrap_or(0.0), 1000.0) / 5.0;
        let grains = capped_score(nv.whole_grains_g.unwrap_or(0.0), 75.0) / 5.0;
        let sodium = if nv.sodium_mg.unwrap_or(f64::INFINITY) <= 1500.0 {
            2.0
        } else if nv.sodium_mg.unwrap_or(0.0) >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium_mg.unwrap_or(0.0)) / (2300.0 - 1500.0) * 2.0
        };
        veg + fruit + dairy + grains + sodium
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
