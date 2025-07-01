use dietarycodex::nutrition_vector::NutritionVector;
use dietarycodex::unmapped_monitor::UNMAPPED_MONITOR;
use serde_json::Value;
use std::collections::HashMap;

#[test]
fn logs_unknown_field() {
    UNMAPPED_MONITOR.reset();
    let mut map = HashMap::new();
    map.insert("venison_g".to_string(), Value::from(50.0));
    let (_nv, _trace) = NutritionVector::from_partial_map(&map);
    let snap = UNMAPPED_MONITOR.snapshot();
    assert!(snap.contains_key("venison_g"));
    assert_eq!(snap["venison_g"].count, 1);
}
