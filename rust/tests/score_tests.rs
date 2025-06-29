use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::scores::hei::HeiScorer;
use dietarycodex::scores::DietScore;

#[test]
fn hei_score_not_nan() {
    let nv = NutritionVector {
        total_fruits_g: 150.0,
        whole_grains_g: 80.0,
        sodium_mg: 1600.0,
        ..Default::default()
    };
    let scorer = HeiScorer;
    let val = scorer.score(&nv);
    assert!(!val.is_nan());
}
