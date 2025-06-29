use super::{DietScore, FieldDeps};
use crate::nutrition_vector::NutritionVector;

pub struct DiiScorer;

impl FieldDeps for DiiScorer {
    fn name() -> &'static str {
        "DII"
    }

    fn required_fields() -> &'static [&'static str] {
        &[
            "saturated_fat_g",
            "trans_fat_g",
            "sugar_g",
            "fiber_g",
            "vitamin_c_mg",
            "vitamin_a_mcg",
            "vitamin_e_mg",
            "omega3_g",
            "zinc_mg",
            "selenium_mcg",
            "magnesium_mg",
        ]
    }
}

impl DietScore for DiiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        // Placeholder weighted scoring reflecting inflammatory potential.
        let mut score = 0.0;
        // pro-inflammatory components
        score += nv.saturated_fat_g.unwrap_or(0.0) * 0.1;
        score += nv.trans_fat_g.unwrap_or(0.0) * 0.3;
        score += nv.sugar_g.unwrap_or(0.0) * 0.05;
        // anti-inflammatory components
        score -= nv.fiber_g.unwrap_or(0.0) * 0.1;
        score -= nv.vitamin_c_mg.unwrap_or(0.0) * 0.005;
        score -= nv.vitamin_a_mcg.unwrap_or(0.0) * 0.0001;
        score -= nv.vitamin_e_mg.unwrap_or(0.0) * 0.02;
        score -= nv.omega3_g.unwrap_or(0.0) * 0.3;
        score -= nv.zinc_mg.unwrap_or(0.0) * 0.05;
        score -= nv.selenium_mcg.unwrap_or(0.0) * 0.003;
        score -= nv.magnesium_mg.unwrap_or(0.0) * 0.01;
        score
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
