use dietarycodex::eval::evaluate_all_scores;
use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::scores::amed::AMedScorer;
use dietarycodex::scores::dash::DashScorer;
use dietarycodex::scores::dashi::DashiScorer;
use dietarycodex::scores::hei::HeiScorer;
use dietarycodex::scores::DietScore;
use dietarycodex::scores::dii::DiiScorer;
use dietarycodex::scores::acs2020::Acs2020Scorer;
use dietarycodex::scores::mind::MindScorer;
use dietarycodex::scores::phdi::PhdiScorer;

fn expected_names() -> Vec<String> {
    dietarycodex::register_scores!()
        .into_iter()
        .map(|s| s.name().to_string())
        .collect()
}

#[test]
fn hei_score_not_nan() {
    let nv = NutritionVector {
        total_fruits_g: 150.0,
        whole_grains_g: 80.0,
        sodium_mg: 1600.0,
        ..Default::default()
    };
    let scorer = HeiScorer;
    let val = scorer.evaluate(&nv);
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
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn dashi_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: 250.0,
        total_fruits_g: 250.0,
        whole_grains_g: 80.0,
        calcium_mg: 900.0,
        sodium_mg: 1600.0,
        ..Default::default()
    };
    let scorer = DashiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn amed_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: 250.0,
        legumes_g: 100.0,
        total_fruits_g: 150.0,
        whole_grains_g: 80.0,
        fish_g: 80.0,
        mono_fat_g: 30.0,
        red_meat_g: 50.0,
        ..Default::default()
    };
    let scorer = AMedScorer;
    let val = scorer.evaluate(&nv);
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
    assert!(scores.scores.contains_key("DASH"));
}

#[test]
fn evaluate_returns_dashi() {
    let nv = NutritionVector::default();
    let scores = evaluate_all_scores(&nv);
    assert!(scores.scores.contains_key("DASHI"));
}

#[test]
fn evaluate_returns_dii() {
    let nv = NutritionVector {
        saturated_fat_g: 8.0,
        sugar_g: 50.0,
        fiber_g: 20.0,
        vitamin_c_mg: 60.0,
        vitamin_a_mcg: 700.0,
        vitamin_e_mg: 10.0,
        omega3_g: 1.0,
        zinc_mg: 12.0,
        selenium_mcg: 55.0,
        magnesium_mg: 300.0,
        trans_fat_g: 0.5,
        ..Default::default()
    };
    let scores = evaluate_all_scores(&nv);
    assert!(scores.scores.contains_key("DII"));
    let scorer = DiiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn acs2020_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: 250.0,
        legumes_g: 90.0,
        total_fruits_g: 180.0,
        whole_grains_g: 80.0,
        red_meat_g: 40.0,
        sugar_g: 30.0,
        alcohol_g: 10.0,
        ..Default::default()
    };
    let scorer = Acs2020Scorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_acs2020() {
    let nv = NutritionVector::default();
    let scores = evaluate_all_scores(&nv);
    assert!(scores.scores.contains_key("ACS2020"));
}

#[test]
fn phdi_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: 250.0,
        legumes_g: 80.0,
        whole_grains_g: 70.0,
        fat_g: 40.0,
        saturated_fat_g: 10.0,
        trans_fat_g: 0.5,
        red_meat_g: 20.0,
        sugar_g: 30.0,
        refined_grains_g: 100.0,
        energy_kcal: 2000.0,
        ..Default::default()
    };
    let scorer = PhdiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_phdi() {
    let nv = NutritionVector::default();
    let scores = evaluate_all_scores(&nv);
    assert!(scores.scores.contains_key("PHDI"));
}

#[test]
fn mind_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: 150.0,
        berries_g: 40.0,
        nuts_g: 20.0,
        whole_grains_g: 60.0,
        fish_g: 50.0,
        poultry_g: 80.0,
        mono_fat_g: 25.0,
        red_meat_g: 20.0,
        sugar_g: 15.0,
        cheese_g: 10.0,
        butter_g: 5.0,
        fast_food_g: 0.0,
        ..Default::default()
    };
    let scorer = MindScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_mind() {
    let nv = NutritionVector::default();
    let scores = evaluate_all_scores(&nv);
    assert!(scores.scores.contains_key("MIND"));
}

#[test]
fn default_evaluates_all_scores() {
    let nv = NutritionVector::default();
    let result = evaluate_all_scores(&nv);
    let expected = expected_names();
    for name in &expected {
        assert!(result.scores.contains_key(name.as_str()), "missing {name}");
    }
    assert_eq!(result.ordered_names, expected);
}
