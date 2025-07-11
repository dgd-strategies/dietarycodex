use super::{DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct Ahei;

impl FieldDeps for Ahei {
    fn name() -> &'static str {
        "AHEI"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("AHEI")
    }
}

impl DietScore for Ahei {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        // Simplified scoring: ratio of healthy components
        let mut score = 0.0;
        if nv.fiber.unwrap_or(0.0) >= 25.0 {
            score += 10.0;
        } else {
            score += nv.fiber.unwrap_or(0.0) / 2.5;
        }
        if nv.fat.unwrap_or(0.0) > 0.0 {
            let puf_ratio = (nv.fat.unwrap_or(0.0) - nv.saturated_fat.unwrap_or(0.0))
                / nv.fat.unwrap_or(0.0);
            score += (puf_ratio * 10.0).clamp(0.0, 10.0);
        }
        score
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
