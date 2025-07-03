use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::eval::evaluate_allow_partial;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn raw_item_only_resolves() {
    let mut map = HashMap::new();
    map.insert("beef_g".to_string(), Value::from(100.0));
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert_eq!(nv.protein, Some(26.0));
    assert!(trace
        .translation_log
        .get("protein")
        .unwrap()
        .source
        .contains(&"beef_g".to_string()));
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().all(|s| s.value.is_none()));
}

#[test]
fn raw_and_canonical_combined() {
    let mut map = HashMap::new();
    map.insert("beef_g".to_string(), Value::from(100.0));
    map.insert("protein".to_string(), Value::from(10.0));
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert_eq!(nv.protein, Some(36.0));
    assert_eq!(trace.translation_log.get("protein").unwrap().value, 26.0);
}

#[test]
fn empty_input_gives_errors() {
    let map: HashMap<String, Value> = HashMap::new();
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert!(trace.missing_fields.contains(&"energy"));
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().all(|s| s.value.is_none()));
    assert!(!result.errors.is_empty());
}
