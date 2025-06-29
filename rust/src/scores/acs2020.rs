use super::{capped_score, DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct Acs2020Scorer;

impl FieldDeps for Acs2020Scorer {
    fn name() -> &'static str {
        "ACS2020"
    }

    fn required_fields() -> &'static [&'static str] {
        &[
            "alcohol_g",
            "legumes_g",
            "red_meat_g",
            "sugar_g",
            "total_fruits_g",
            "vegetables_g",
            "whole_grains_g",
        ]
    }
}

impl DietScore for Acs2020Scorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g.unwrap_or(0.0), 300.0);
        let fruit = capped_score(nv.total_fruits_g.unwrap_or(0.0), 200.0);
        let legumes = capped_score(nv.legumes_g.unwrap_or(0.0), 100.0);
        let grains = capped_score(nv.whole_grains_g.unwrap_or(0.0), 75.0);
        let red_meat = (10.0 - capped_score(nv.red_meat_g.unwrap_or(0.0), 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar_g.unwrap_or(0.0), 50.0)).clamp(0.0, 10.0);
        let alcohol = (10.0 - capped_score(nv.alcohol_g.unwrap_or(0.0), 20.0)).clamp(0.0, 10.0);
        veg + fruit + legumes + grains + red_meat + sugar + alcohol
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
