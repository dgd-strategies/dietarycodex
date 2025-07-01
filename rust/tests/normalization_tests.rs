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
    map.insert("protein_g".to_string(), Value::from(50.0));
    map.insert("sodium_mg".to_string(), Value::from(1500.0));
    map.insert("calcium_mg".to_string(), Value::from(900.0));
    map.insert("iron_mg".to_string(), Value::from(10.0));
    map.insert("vitamin_c_mg".to_string(), Value::from(60.0));
    map.insert("total_fruits_g".to_string(), Value::from(200.0));
    map.insert("vegetables_g".to_string(), Value::from(250.0));
    map.insert("whole_grains_g".to_string(), Value::from(90.0));
    map.insert("refined_grains_g".to_string(), Value::from(80.0));
    map.insert("legumes_g".to_string(), Value::from(30.0));
    map.insert("fish_g".to_string(), Value::from(20.0));
    map.insert("red_meat_g".to_string(), Value::from(30.0));
    map.insert("mono_fat_g".to_string(), Value::from(25.0));
    map.insert("berries_g".to_string(), Value::from(5.0));
    map.insert("cheese_g".to_string(), Value::from(15.0));
    map.insert("butter_g".to_string(), Value::from(5.0));
    map.insert("poultry_g".to_string(), Value::from(40.0));
    map.insert("fast_food_g".to_string(), Value::from(0.0));
    map.insert("nuts_g".to_string(), Value::from(10.0));
    map.insert("omega3_g".to_string(), Value::from(1.0));
    map.insert("vitamin_a_mcg".to_string(), Value::from(800.0));
    map.insert("vitamin_e_mg".to_string(), Value::from(10.0));
    map.insert("zinc_mg".to_string(), Value::from(12.0));
    map.insert("selenium_mcg".to_string(), Value::from(55.0));
    map.insert("magnesium_mg".to_string(), Value::from(300.0));
    map.insert("trans_fat_g".to_string(), Value::from(0.1));
    map.insert("alcohol_intake".to_string(), Value::from(2.0));

    let (nv, _trace) = NutritionVector::from_partial_map(&map);
    let result = evaluate_allow_partial(&nv);
    assert!(result.scores.values().any(|s| s.value.is_some()));
}

#[test]
fn duplicate_fields_prefers_canonical() {
    let mut map = HashMap::new();
    map.insert("carbs_g".to_string(), Value::from(100.0));
    map.insert("carbohydrate".to_string(), Value::from(150.0));
    let (nv, trace) = NutritionVector::from_partial_map(&map);
    assert_eq!(nv.carbs_g, Some(100.0));
    assert!(trace.conflicting_aliases.contains(&("carbohydrate".to_string(), "carbs_g")));
}

#[test]
fn missing_canonical_field_errors() {
    let mut map: HashMap<String, f64> = HashMap::new();
    for f in NutritionVector::all_field_names() {
        if *f != "fat_g" {
            map.insert(f.to_string(), 1.0);
        }
    }
    let err = NutritionVector::from_map(&map).unwrap_err();
    assert!(err.missing_canonical_fields.contains(&"fat_g"));
}
