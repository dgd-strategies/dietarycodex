use dietarycodex::eval::evaluate_all_scores;
use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::scores::dash::DashScorer;
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

#[test]
fn dash_score_not_nan() {
    let nv = NutritionVector {
        total_fruits_g: 200.0,
        vegetables_g: 250.0,
        whole_grains_g: 80.0,
        sodium_mg: 1600.0,
        saturated_fat_g: 10.0,
        energy_kcal: 2000.0,
        ..Default::default()
    };
    let scorer = DashScorer;
    let val = scorer.score(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_dash() {
    let nv = NutritionVector {
        total_fruits_g: 200.0,
        vegetables_g: 200.0,
        whole_grains_g: 80.0,
        sodium_mg: 1600.0,
        saturated_fat_g: 10.0,
        energy_kcal: 2000.0,
        ..Default::default()
    };
    let scores = evaluate_all_scores(&nv);
    assert!(scores.contains_key("DASH"));
}
