use super::{DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct DiiScorer;

impl FieldDeps for DiiScorer {
    fn name() -> &'static str {
        "DII"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("DII")
    }
}

impl DietScore for DiiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        // Placeholder weighted scoring reflecting inflammatory potential.
        let mut score = 0.0;
        // pro-inflammatory components
        score += nv.saturated_fat.unwrap_or(0.0) * 0.1;
        score += nv.trans_fat.unwrap_or(0.0) * 0.3;
        score += nv.sugar.unwrap_or(0.0) * 0.05;
        // anti-inflammatory components
        score -= nv.fiber.unwrap_or(0.0) * 0.1;
        score -= nv.vitamin_c.unwrap_or(0.0) * 0.005;
        score -= nv.vitamin_a.unwrap_or(0.0) * 0.0001;
        score -= nv.vitamin_e.unwrap_or(0.0) * 0.02;
        score -= nv.omega3.unwrap_or(0.0) * 0.3;
        score -= nv.zinc.unwrap_or(0.0) * 0.05;
        score -= nv.selenium.unwrap_or(0.0) * 0.003;
        score -= nv.magnesium.unwrap_or(0.0) * 0.01;
        score
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
