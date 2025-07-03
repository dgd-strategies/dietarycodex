use dietarycodex::eval::{
    evaluate_all_scores, evaluate_allow_partial, format_skipped_scores,
    print_scores_as_json_allow_partial, ScoreInfo,
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
        total_fruits: Some(150.0),
        whole_grains: Some(80.0),
        sodium: Some(1600.0),
        ..Default::default()
    };
    let scorer = HeiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn dash_score_not_nan() {
    let nv = NutritionVector {
        total_fruits: Some(200.0),
        vegetables: Some(250.0),
        whole_grains: Some(80.0),
        sodium: Some(1600.0),
        saturated_fat: Some(10.0),
        energy: Some(2000.0),
        ..Default::default()
    };
    let scorer = DashScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn dashi_score_not_nan() {
    let nv = NutritionVector {
        vegetables: Some(250.0),
        total_fruits: Some(250.0),
        whole_grains: Some(80.0),
        calcium: Some(900.0),
        sodium: Some(1600.0),
        ..Default::default()
    };
    let scorer = DashiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn amed_score_not_nan() {
    let nv = NutritionVector {
        vegetables: Some(250.0),
        legumes: Some(100.0),
        total_fruits: Some(150.0),
        whole_grains: Some(80.0),
        fish: Some(80.0),
        mono_fat: Some(30.0),
        red_meat: Some(50.0),
        ..Default::default()
    };
    let scorer = AMedScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn evaluate_returns_dash() {
    let nv = NutritionVector {
        total_fruits: Some(200.0),
        vegetables: Some(200.0),
        whole_grains: Some(80.0),
        sodium: Some(1600.0),
        saturated_fat: Some(10.0),
        energy: Some(2000.0),
        ..Default::default()
    };
    let scores = evaluate_allow_partial(&nv);
    match scores.scores.get("DASH") {
        Some(info) if info.value.is_some() => {}
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
        saturated_fat: Some(8.0),
        sugar: Some(50.0),
        fiber: Some(20.0),
        vitamin_c: Some(60.0),
        vitamin_a: Some(700.0),
        vitamin_e: Some(10.0),
        omega3: Some(1.0),
        zinc: Some(12.0),
        selenium: Some(55.0),
        magnesium: Some(300.0),
        trans_fat: Some(0.5),
        ..Default::default()
    };
    let scores = evaluate_allow_partial(&nv);
    match scores.scores.get("DII") {
        Some(info) if info.value.is_some() => {}
        _ => panic!("DII score not computed"),
    }
    let scorer = DiiScorer;
    let val = scorer.evaluate(&nv);
    assert!(!val.is_nan());
}

#[test]
fn acs2020_score_not_nan() {
    let nv = NutritionVector {
        vegetables: Some(250.0),
        legumes: Some(90.0),
        total_fruits: Some(180.0),
        whole_grains: Some(80.0),
        red_meat: Some(40.0),
        sugar: Some(30.0),
        alcohol: Some(10.0),
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
        vegetables: Some(250.0),
        legumes: Some(80.0),
        whole_grains: Some(70.0),
        fat: Some(40.0),
        saturated_fat: Some(10.0),
        trans_fat: Some(0.5),
        red_meat: Some(20.0),
        sugar: Some(30.0),
        refined_grains: Some(100.0),
        energy: Some(2000.0),
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
        vegetables: Some(150.0),
        berries: Some(40.0),
        nuts: Some(20.0),
        whole_grains: Some(60.0),
        fish: Some(50.0),
        poultry: Some(80.0),
        mono_fat: Some(25.0),
        red_meat: Some(20.0),
        sugar: Some(15.0),
        cheese: Some(10.0),
        butter: Some(5.0),
        fast_food: Some(0.0),
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
        fiber: Some(5.0),
        ..Default::default()
    };
    let err = evaluate_all_scores(&nv).unwrap_err();
    assert!(err.missing_canonical_fields.contains(&"fat"));
    assert!(err.missing_canonical_fields.contains(&"saturated_fat"));
}

#[test]
fn allow_partial_skips_missing() {
    let nv = NutritionVector {
        fiber: Some(5.0),
        fat: Some(40.0),
        saturated_fat: Some(10.0),
        ..Default::default()
    };
    let result = evaluate_allow_partial(&nv);
    match result.scores.get("AHEI") {
        Some(info) if info.value.is_some() => {}
        _ => panic!("AHEI should be computed"),
    }
    match result.scores.get("DASH") {
        Some(info) if info.value.is_none() => {}
        _ => panic!("DASH should be skipped"),
    }
}

#[test]
fn skipped_reason_lists_missing_fields() {
    let nv = NutritionVector {
        fiber: Some(5.0),
        ..Default::default()
    };
    let result = evaluate_allow_partial(&nv);
    match result.scores.get("AHEI") {
        Some(info) if info.value.is_none() => {
            let reason = info.explanation.as_ref().unwrap();
            assert!(reason.contains("fat"));
            assert!(reason.contains("saturated_fat"));
        }
        _ => panic!("AHEI should be skipped"),
    }
    match result.scores.get("DASH") {
        Some(info) if info.value.is_none() => {
            let reason = info.explanation.as_ref().unwrap();
            assert!(reason.contains("sodium"));
            assert!(reason.contains("energy"));
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

#[test]
fn all_field_names_match_template_order() {
    let fields = NutritionVector::all_field_names();
    let template = {
        let line = std::fs::read_to_string("../data/template.csv")
            .expect("read template");
        line.lines()
            .next()
            .unwrap()
            .split(',')
            .map(|s| s.trim().to_string())
            .collect::<Vec<String>>()
    };
    let mut canon_in_template = Vec::new();
    for name in template {
        if fields.contains(&name.as_str()) {
            canon_in_template.push(name);
        }
    }
    let from_fn: Vec<&str> = fields.to_vec();
    assert_eq!(from_fn, canon_in_template.iter().map(|s| s.as_str()).collect::<Vec<_>>());
}

fn all_fields_nv() -> NutritionVector {
    NutritionVector {
        energy: Some(2000.0),
        fat: Some(70.0),
        saturated_fat: Some(10.0),
        carbs: Some(250.0),
        fiber: Some(30.0),
        sugar: Some(25.0),
        protein: Some(50.0),
        sodium: Some(1500.0),
        calcium: Some(900.0),
        iron: Some(10.0),
        vitamin_c: Some(60.0),
        total_fruits: Some(250.0),
        vegetables: Some(250.0),
        whole_grains: Some(90.0),
        refined_grains: Some(90.0),
        legumes: Some(100.0),
        fish: Some(50.0),
        red_meat: Some(30.0),
        mono_fat: Some(30.0),
        berries: Some(30.0),
        cheese: Some(20.0),
        butter: Some(5.0),
        poultry: Some(50.0),
        fast_food: Some(0.0),
        nuts: Some(20.0),
        omega3: Some(1.0),
        vitamin_a: Some(800.0),
        vitamin_e: Some(10.0),
        zinc: Some(12.0),
        selenium: Some(55.0),
        magnesium: Some(300.0),
        trans_fat: Some(0.2),
        alcohol: Some(5.0),
    }
}

#[test]
fn all_scores_skipped_when_no_input() {
    let nv = NutritionVector::default();
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().all(|s| s.value.is_none()));
}

#[test]
fn no_scores_skipped_when_all_fields_present() {
    let nv = all_fields_nv();
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().all(|s| s.value.is_some()));
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

    for (name, info) in result.scores {
        if info.value.is_none() {
            let reason = info.explanation.unwrap();
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

#[test]
fn skipped_reason_field_order() {
    let nv = NutritionVector::default();
    let result = evaluate_allow_partial(&nv);
    for status in result.scores.values() {
        if status.value.is_none() {
            let reason = status.explanation.as_ref().unwrap();
            let fields: Vec<&str> = reason
                .trim_start_matches("missing fields: ")
                .split(',')
                .map(|s| s.trim())
                .collect();
            let mut sorted = fields.clone();
            sorted.sort();
            assert_eq!(fields, sorted);
        }
    }
}

#[test]
fn verbose_partial_output_stable() {
    let nv = NutritionVector::default();
    let result = evaluate_allow_partial(&nv);
    let output = format_skipped_scores(&result).expect("expected skipped scores");
    let lines: Vec<&str> = output.lines().collect();
    assert!(!lines.is_empty());
    assert_eq!(lines[0], "Skipped scores:");
    let names: Vec<&str> = lines
        .iter()
        .skip(1)
        .map(|l| l.split(':').next().unwrap().trim())
        .collect();
    let mut sorted_names = names.clone();
    sorted_names.sort();
    assert_eq!(names, sorted_names);
    for line in lines.iter().skip(1) {
        let reason = line.split(':').nth(1).unwrap().trim();
        let fields: Vec<&str> = reason
            .trim_start_matches("missing fields: ")
            .split(',')
            .map(|s| s.trim())
            .collect();
        let mut sorted_fields = fields.clone();
        sorted_fields.sort();
        assert_eq!(fields, sorted_fields);
    }
}

#[test]
fn deserialize_alias_alcohol() {
    let json = r#"{"ALCOHOL": 12.5}"#;
    let nv: NutritionVector = serde_json::from_str(json).unwrap();
    assert_eq!(nv.alcohol, Some(12.5));
}

#[test]
fn deserialize_alias_alcohol_intake() {
    let json = r#"{"alcohol_intake": 3.0}"#;
    let nv: NutritionVector = serde_json::from_str(json).unwrap();
    assert_eq!(nv.alcohol, Some(3.0));
}
