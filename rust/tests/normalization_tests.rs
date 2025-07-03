use dietarycodex::eval::evaluate_allow_partial;
use dietarycodex::nutrition_vector::NutritionVector;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn noisy_aliases_normalize_and_score() {
    let mut map = HashMap::new();
    map.insert("kcal".to_string(), Value::from(1800.0));
    map.insert("fat".to_string(), Value::from(60.0));
    map.insert("saturated_fat".to_string(), Value::from(15.0));
    map.insert("carbohydrate".to_string(), Value::from(200.0));
    map.insert("fiber".to_string(), Value::from(25.0));
    map.insert("sugars".to_string(), Value::from(40.0));
    map.insert("protein".to_string(), Value::from(50.0));
    map.insert("sodium".to_string(), Value::from(1500.0));
    map.insert("calcium".to_string(), Value::from(900.0));
    map.insert("iron".to_string(), Value::from(10.0));
    map.insert("vitamin_c".to_string(), Value::from(60.0));
    map.insert("total_fruits".to_string(), Value::from(200.0));
    map.insert("vegetables".to_string(), Value::from(250.0));
    map.insert("whole_grains".to_string(), Value::from(90.0));
    map.insert("refined_grains".to_string(), Value::from(80.0));
    map.insert("legumes".to_string(), Value::from(30.0));
    map.insert("fish".to_string(), Value::from(20.0));
    map.insert("red_meat".to_string(), Value::from(30.0));
    map.insert("mono_fat".to_string(), Value::from(25.0));
    map.insert("berries".to_string(), Value::from(5.0));
    map.insert("cheese".to_string(), Value::from(15.0));
    map.insert("butter".to_string(), Value::from(5.0));
    map.insert("poultry".to_string(), Value::from(40.0));
    map.insert("fast_food".to_string(), Value::from(0.0));
    map.insert("nuts".to_string(), Value::from(10.0));
    map.insert("omega3".to_string(), Value::from(1.0));
    map.insert("vitamin_a".to_string(), Value::from(800.0));
    map.insert("vitamin_e".to_string(), Value::from(10.0));
    map.insert("zinc".to_string(), Value::from(12.0));
    map.insert("selenium".to_string(), Value::from(55.0));
    map.insert("magnesium".to_string(), Value::from(300.0));
    map.insert("trans_fat".to_string(), Value::from(0.1));
    map.insert("alcohol_intake".to_string(), Value::from(2.0));

    let (nv, _trace) = NutritionVector::from_partial_map(&map);
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().any(|s| s.value.is_some()));
}

#[test]
fn duplicate_fields_prefers_canonical() {
    let mut map = HashMap::new();
    map.insert("carbs".to_string(), Value::from(100.0));
    map.insert("carbohydrate".to_string(), Value::from(150.0));
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert_eq!(nv.carbs, Some(100.0));
    assert!(trace.conflicting_aliases.contains(&("carbohydrate".to_string(), "carbs")));
}

#[test]
fn missing_canonical_field_errors() {
    let mut map: HashMap<String, f64> = HashMap::new();
    for f in NutritionVector::all_field_names() {
        if *f != "fat" {
            map.insert(f.to_string(), 1.0);
        }
    }
    let err = NutritionVector::from_map(&map).unwrap_err();
    assert!(err.missing_canonical_fields.contains(&"fat"));
}
