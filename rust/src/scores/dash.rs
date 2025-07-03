use super::{DietScore, FieldDeps};
use crate::contracts;
use crate::nutrition_vector::NutritionVector;

pub struct DashScorer;

impl FieldDeps for DashScorer {
    fn name() -> &'static str {
        "DASH"
    }

    fn required_fields() -> &'static [&'static str] {
        contracts::required_fields("DASH")
    }
}

impl DietScore for DashScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let fruit = (nv.total_fruits.unwrap_or(0.0) / 400.0 * 10.0).clamp(0.0, 10.0);
        let veg = (nv.vegetables.unwrap_or(0.0) / 400.0 * 10.0).clamp(0.0, 10.0);
        let grains = (nv.whole_grains.unwrap_or(0.0) / 75.0 * 10.0).clamp(0.0, 10.0);
        let sodium = if nv.sodium.unwrap_or(f64::INFINITY) <= 1500.0 {
            10.0
        } else if nv.sodium.unwrap_or(0.0) >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium.unwrap_or(0.0)) / (2300.0 - 1500.0) * 10.0
        };
        let sat_percent = if nv.energy.unwrap_or(0.0) > 0.0 {
            (nv.saturated_fat.unwrap_or(0.0) * 9.0) / nv.energy.unwrap_or(0.0) * 100.0
        } else {
            0.0
        };
        let sat_fat = if sat_percent <= 5.0 {
            10.0
        } else if sat_percent >= 15.0 {
            0.0
        } else {
            (15.0 - sat_percent) / (15.0 - 5.0) * 10.0
        };
        fruit + veg + grains + sodium + sat_fat
    }

    fn name(&self) -> &'static str {
        <Self as FieldDeps>::name()
    }

    fn required_fields(&self) -> &'static [&'static str] {
        <Self as FieldDeps>::required_fields()
    }
}
