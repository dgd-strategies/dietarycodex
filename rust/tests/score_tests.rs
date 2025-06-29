use dietarycodex::eval::{
    evaluate_all_scores, evaluate_allow_partial, print_scores_as_json_allow_partial, ScorerStatus,
};
use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::scores::acs2020::Acs2020Scorer;
use dietarycodex::scores::amed::AMedScorer;
use dietarycodex::scores::dash::DashScorer;
use dietarycodex::scores::dashi::DashiScorer;
use dietarycodex::scores::dii::DiiScorer;
use dietarycodex::scores::hei::HeiScorer;
use dietarycodex::scores::mind::MindScorer;
use dietarycodex::scores::phdi::PhdiScorer;
use dietarycodex::scores::DietScore;
use serde_json;

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
    match scores.scores.get("DASH") {
        Some(ScorerStatus::Complete(_)) => {}
        _ => panic!("DASH score not computed"),
    }
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
    match scores.scores.get("DII") {
        Some(ScorerStatus::Complete(_)) => {}
        _ => panic!("DII score not computed"),
    }
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
    match result.scores.get("AHEI") {
        Some(ScorerStatus::Complete(_)) => {}
        _ => panic!("AHEI should be computed"),
    }
    match result.scores.get("DASH") {
        Some(ScorerStatus::Skipped { .. }) => {}
        _ => panic!("DASH should be skipped"),
    }
}

#[test]
fn skipped_reason_lists_missing_fields() {
    let nv = NutritionVector {
        fiber_g: Some(5.0),
        ..Default::default()
    };
    let result = evaluate_allow_partial(&nv);
    match result.scores.get("AHEI") {
        Some(ScorerStatus::Skipped { reason }) => {
            assert!(reason.contains("fat_g"));
            assert!(reason.contains("saturated_fat_g"));
        }
        _ => panic!("AHEI should be skipped"),
    }
    match result.scores.get("DASH") {
        Some(ScorerStatus::Skipped { reason }) => {
            assert!(reason.contains("sodium_mg"));
            assert!(reason.contains("energy_kcal"));
        }
        _ => panic!("DASH should be skipped"),
    }
}

#[test]
fn metadata_fields_are_valid() {
    use dietarycodex::scores::registry::all_score_metadata;
    use std::collections::HashSet;

    let all_fields: HashSet<&str> = NutritionVector::all_field_names().iter().copied().collect();
    for meta in all_score_metadata() {
        for field in meta.required_fields {
            assert!(
                all_fields.contains(field),
                "{} missing field {}",
                meta.name,
                field
            );
        }
    }
}

#[test]
fn metadata_sorted() {
    use dietarycodex::scores::registry::all_score_metadata;

    let metas = all_score_metadata();
    let mut names: Vec<&str> = metas.iter().map(|m| m.name).collect();
    let mut sorted_names = names.clone();
    sorted_names.sort();
    assert_eq!(names, sorted_names, "score metadata not sorted by name");

    for meta in metas {
        let mut fields: Vec<&str> = meta.required_fields.iter().copied().collect();
        let mut sorted_fields = fields.clone();
        sorted_fields.sort();
        assert_eq!(fields, sorted_fields, "fields for {} not sorted", meta.name);
    }
}

fn all_fields_nv() -> NutritionVector {
    NutritionVector {
        energy_kcal: Some(2000.0),
        fat_g: Some(70.0),
        saturated_fat_g: Some(10.0),
        carbs_g: Some(250.0),
        fiber_g: Some(30.0),
        sugar_g: Some(25.0),
        protein_g: Some(50.0),
        sodium_mg: Some(1500.0),
        calcium_mg: Some(900.0),
        iron_mg: Some(10.0),
        vitamin_c_mg: Some(60.0),
        total_fruits_g: Some(250.0),
        vegetables_g: Some(250.0),
        whole_grains_g: Some(90.0),
        refined_grains_g: Some(90.0),
        legumes_g: Some(100.0),
        fish_g: Some(50.0),
        red_meat_g: Some(30.0),
        mono_fat_g: Some(30.0),
        berries_g: Some(30.0),
        cheese_g: Some(20.0),
        butter_g: Some(5.0),
        poultry_g: Some(50.0),
        fast_food_g: Some(0.0),
        nuts_g: Some(20.0),
        omega3_g: Some(1.0),
        vitamin_a_mcg: Some(800.0),
        vitamin_e_mg: Some(10.0),
        zinc_mg: Some(12.0),
        selenium_mcg: Some(55.0),
        magnesium_mg: Some(300.0),
        trans_fat_g: Some(0.2),
        alcohol_g: Some(5.0),
    }
}

#[test]
fn all_scores_skipped_when_no_input() {
    let nv = NutritionVector::default();
    let result = evaluate_allow_partial(&nv);
    assert!(result
        .scores
        .values()
        .all(|s| matches!(s, ScorerStatus::Skipped { .. })));
}

#[test]
fn no_scores_skipped_when_all_fields_present() {
    let nv = all_fields_nv();
    let result = evaluate_allow_partial(&nv);
    assert!(result
        .scores
        .values()
        .all(|s| matches!(s, ScorerStatus::Complete(_))));
}

#[test]
fn skipped_reason_fields_are_required() {
    use dietarycodex::scores::registry::all_score_metadata;
    use std::collections::HashMap;

    let nv = NutritionVector::default();
    let result = evaluate_allow_partial(&nv);
    let meta_map: HashMap<&str, Vec<&str>> = all_score_metadata()
        .into_iter()
        .map(|m| (m.name, m.required_fields.to_vec()))
        .collect();

    for (name, status) in result.scores {
        if let ScorerStatus::Skipped { reason } = status {
            let fields: Vec<&str> = reason
                .trim_start_matches("missing fields: ")
                .split(',')
                .map(|s| s.trim())
                .collect();
            let required = meta_map.get(name.as_str()).unwrap();
            for f in fields {
                assert!(
                    required.contains(&f),
                    "{} reported missing {} not in required fields",
                    name,
                    f
                );
            }
        }
    }
}

#[test]
fn json_output_valid_with_skipped_scores() {
    let nv = NutritionVector::default();
    let json = print_scores_as_json_allow_partial(&nv);
    let parsed: serde_json::Value = serde_json::from_str(&json).expect("invalid json");
    assert!(parsed.get("scores").is_some());
}
