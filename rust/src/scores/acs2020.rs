use super::{capped_score, DietScore};
use crate::nutrition_vector::NutritionVector;

pub struct Acs2020Scorer;

impl DietScore for Acs2020Scorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let veg = capped_score(nv.vegetables_g, 300.0);
        let fruit = capped_score(nv.total_fruits_g, 200.0);
        let legumes = capped_score(nv.legumes_g, 100.0);
        let grains = capped_score(nv.whole_grains_g, 75.0);
        let red_meat = (10.0 - capped_score(nv.red_meat_g, 100.0)).clamp(0.0, 10.0);
        let sugar = (10.0 - capped_score(nv.sugar_g, 50.0)).clamp(0.0, 10.0);
        let alcohol = (10.0 - capped_score(nv.alcohol_g, 20.0)).clamp(0.0, 10.0);
        veg + fruit + legumes + grains + red_meat + sugar + alcohol
    }

    fn name(&self) -> &'static str {
        "ACS2020"
    }
}
