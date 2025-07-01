use dietarycodex::eval::evaluate_allow_partial;
use dietarycodex::nutrition_vector::NutritionVector;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn trace_consistency() {
    let mut map = HashMap::new();
    map.insert("kcal".to_string(), Value::from(2000.0));
    map.insert("fat_g".to_string(), Value::from(70.0));
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert!(trace
        .aliases_applied
        .contains(&("kcal".to_string(), "energy_kcal")));

    let mut result = evaluate_allow_partial(&nv);
    result.trace.aliases_applied = trace.aliases_applied;

    for f in &result.trace.used_fields {
        assert!(
            !result.trace.missing_fields.contains(f),
            "{} listed as used and missing",
            f
        );
    }

    let dash_reason = result
        .scores
        .get("DASH")
        .unwrap()
        .explanation
        .as_ref()
        .unwrap();
    assert!(dash_reason.contains("saturated_fat_g"));
}

#[test]
fn partial_map_conflicting_alias() {
    let mut map = HashMap::new();
    map.insert("alcohol_g".to_string(), Value::from(10.0));
    map.insert("alc".to_string(), Value::from(5.0));
    let (_nv, trace) = NutritionVector::from_partial_map(&map);
    assert!(trace
        .conflicting_aliases
        .contains(&("alc".to_string(), "alcohol_g")));
}
