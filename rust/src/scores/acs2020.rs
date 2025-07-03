use super::{capped_score, DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct Acs2020Scorer;

impl FieldDeps for Acs2020Scorer {
    fn name() -> &'static str {
        "ACS2020"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("ACS2020")
    }
}

impl DietScore for Acs2020Scorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables.unwrap_or(0.0), 300.0);
        let fruit = capped_score(nv.total_fruits.unwrap_or(0.0), 200.0);
        let legumes = capped_score(nv.legumes.unwrap_or(0.0), 100.0);
        let grains = capped_score(nv.whole_grains.unwrap_or(0.0), 75.0);
        let red_meat = (10.0 - capped_score(nv.red_meat.unwrap_or(0.0), 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar.unwrap_or(0.0), 50.0)).clamp(0.0, 10.0);
        let alcohol = (10.0 - capped_score(nv.alcohol.unwrap_or(0.0), 20.0)).clamp(0.0, 10.0);
        veg + fruit + legumes + grains + red_meat + sugar + alcohol
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
