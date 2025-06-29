use super::DietScore;
use crate::nutrition_vector::NutritionVector;

pub struct HeiScorer;

impl DietScore for HeiScorer {
    fn evaluate(&self, nv: &NutritionVector) -> f64 {
        let mut score = 0.0;
        // fruit component: 0-10 points with 200g threshold
        score += (nv.total_fruits_g / 200.0 * 10.0).clamp(0.0, 10.0);
        // whole grains component: 0-10 points with 75g threshold
        score += (nv.whole_grains_g / 75.0 * 10.0).clamp(0.0, 10.0);
        // sodium moderation: linear 10 points at <=1500 mg down to 0 at 2300 mg
        let sodium_score = if nv.sodium_mg <= 1500.0 {
            10.0
        } else if nv.sodium_mg >= 2300.0 {
            0.0
        } else {
            (2300.0 - nv.sodium_mg) / (2300.0 - 1500.0) * 10.0
        };
        score + sodium_score
    }

    fn name(&self) -> &'static str {
        "HEI"
    }
}
