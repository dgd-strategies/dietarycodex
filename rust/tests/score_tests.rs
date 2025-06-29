use dietarycodex::eval::{evaluate_all_scores, evaluate_allow_partial};
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
        total_fruits_g: Some(150.0),
        whole_grains_g: Some(80.0),
        sodium_mg: Some(1600.0),
        ..Default::default()
    };
    let scorer = HeiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn dash_score_not_nan() {
    let nv = NutritionVector {
        total_fruits_g: Some(200.0),
        vegetables_g: Some(250.0),
        whole_grains_g: Some(80.0),
        sodium_mg: Some(1600.0),
        saturated_fat_g: Some(10.0),
        energy_kcal: Some(2000.0),
        ..Default::default()
    };
    let scorer = DashScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn dashi_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: Some(250.0),
        total_fruits_g: Some(250.0),
        whole_grains_g: Some(80.0),
        calcium_mg: Some(900.0),
        sodium_mg: Some(1600.0),
        ..Default::default()
    };
    let scorer = DashiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn amed_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: Some(250.0),
        legumes_g: Some(100.0),
        total_fruits_g: Some(150.0),
        whole_grains_g: Some(80.0),
        fish_g: Some(80.0),
        mono_fat_g: Some(30.0),
        red_meat_g: Some(50.0),
        ..Default::default()
    };
    let scorer = AMedScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_dash() {
    let nv = NutritionVector {
        total_fruits_g: Some(200.0),
        vegetables_g: Some(200.0),
        whole_grains_g: Some(80.0),
        sodium_mg: Some(1600.0),
        saturated_fat_g: Some(10.0),
        energy_kcal: Some(2000.0),
        ..Default::default()
    };
    let scores = evaluate_allow_partial(&nv);
    assert!(scores.scores.contains_key("DASH"));
}

#[test]
fn evaluate_returns_dashi() {
    let names = expected_names();
    assert!(names.contains(&"DASHI".to_string()));
}

#[test]
fn evaluate_returns_dii() {
    let nv = NutritionVector {
        saturated_fat_g: Some(8.0),
        sugar_g: Some(50.0),
        fiber_g: Some(20.0),
        vitamin_c_mg: Some(60.0),
        vitamin_a_mcg: Some(700.0),
        vitamin_e_mg: Some(10.0),
        omega3_g: Some(1.0),
        zinc_mg: Some(12.0),
        selenium_mcg: Some(55.0),
        magnesium_mg: Some(300.0),
        trans_fat_g: Some(0.5),
        ..Default::default()
    };
    let scores = evaluate_allow_partial(&nv);
    assert!(scores.scores.contains_key("DII"));
    let scorer = DiiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn acs2020_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: Some(250.0),
        legumes_g: Some(90.0),
        total_fruits_g: Some(180.0),
        whole_grains_g: Some(80.0),
        red_meat_g: Some(40.0),
        sugar_g: Some(30.0),
        alcohol_g: Some(10.0),
        ..Default::default()
    };
    let scorer = Acs2020Scorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_acs2020() {
    let names = expected_names();
    assert!(names.contains(&"ACS2020".to_string()));
}

#[test]
fn phdi_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: Some(250.0),
        legumes_g: Some(80.0),
        whole_grains_g: Some(70.0),
        fat_g: Some(40.0),
        saturated_fat_g: Some(10.0),
        trans_fat_g: Some(0.5),
        red_meat_g: Some(20.0),
        sugar_g: Some(30.0),
        refined_grains_g: Some(100.0),
        energy_kcal: Some(2000.0),
        ..Default::default()
    };
    let scorer = PhdiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_phdi() {
    let names = expected_names();
    assert!(names.contains(&"PHDI".to_string()));
}

#[test]
fn mind_score_not_nan() {
    let nv = NutritionVector {
        vegetables_g: Some(150.0),
        berries_g: Some(40.0),
        nuts_g: Some(20.0),
        whole_grains_g: Some(60.0),
        fish_g: Some(50.0),
        poultry_g: Some(80.0),
        mono_fat_g: Some(25.0),
        red_meat_g: Some(20.0),
        sugar_g: Some(15.0),
        cheese_g: Some(10.0),
        butter_g: Some(5.0),
        fast_food_g: Some(0.0),
        ..Default::default()
    };
    let scorer = MindScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_mind() {
    let names = expected_names();
    assert!(names.contains(&"MIND".to_string()));
}

#[test]
fn default_evaluates_all_scores() {
    let nv = NutritionVector::default();
    assert!(evaluate_all_scores(&nv).is_err());
}

#[test]
fn returns_err_with_missing_fields() {
    let nv = NutritionVector {
        fiber_g: Some(5.0),
        ..Default::default()
    };
    let err = evaluate_all_scores(&nv).unwrap_err();
    assert!(err.contains(&"fat_g"));
    assert!(err.contains(&"saturated_fat_g"));
}

#[test]
fn allow_partial_skips_missing() {
    let nv = NutritionVector {
        fiber_g: Some(5.0),
        fat_g: Some(40.0),
        saturated_fat_g: Some(10.0),
        ..Default::default()
    };
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.contains_key("AHEI"));
    assert!(!result.scores.contains_key("DASH"));
}

#[test]
fn metadata_fields_are_valid() {
    use dietarycodex::scores::registry::all_score_metadata;
    use std::collections::HashSet;

    let all_fields: HashSet<&str> = NutritionVector::all_field_names().iter().copied().collect();
    for meta in all_score_metadata() {
        for field in meta.required_fields {
            assert!(all_fields.contains(field), "{} missing field {}", meta.name, field);
        }
    }
}
