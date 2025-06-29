use super::DietScore;
use crate::nutrition_vector::NutritionVector;

pub struct DashScorer;

impl DietScore for DashScorer {
    fn score(&self, nv: &NutritionVector) -> f64 {
        let fruit = (nv.total_fruits_g / 400.0 * 10.0).clamp(0.0, 10.0);
        let veg = (nv.vegetables_g / 400.0 * 10.0).clamp(0.0, 10.0);
        let grains = (nv.whole_grains_g / 75.0 * 10.0).clamp(0.0, 10.0);
        let sodium = if nv.sodium_mg <= 1500.0 {
            10.0
        } else if nv.sodium_mg >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium_mg) / (2300.0 - 1500.0) * 10.0
        };
        let sat_percent = if nv.energy_kcal > 0.0 {
            (nv.saturated_fat_g * 9.0) / nv.energy_kcal * 100.0
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
        "DASH"
    }
}
