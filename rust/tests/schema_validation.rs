use dietarycodex::nutrition_vector::NutritionVector;
use std::collections::HashMap;

fn full_map() -> HashMap<String, f64> {
    let mut m = HashMap::new();
    for field in NutritionVector::all_field_names() {
        m.insert((*field).to_string(), 1.0);
    }
    m
}

#[test]
fn load_canonical_fields() {
    let map = full_map();
    let nv = NutritionVector::from_map(&map).expect("should load");
    assert!(nv.missing_fields().is_empty());
}

#[test]
fn load_with_aliases() {
    let mut map = HashMap::new();
    for field in NutritionVector::all_field_names() {
        let key = match *field {
            "alcohol_g" => "alc",
            "energy_kcal" => "kcal",
            "sodium_mg" => "sodium",
            _ => field,
        };
        map.insert(key.to_string(), 1.0);
    }
    let nv = NutritionVector::from_map(&map).expect("aliases should map");
    assert!(nv.missing_fields().is_empty());
}

#[test]
fn unmapped_field_fails() {
    let mut map = full_map();
    map.remove("alcohol_g");
    map.insert("alcohol_content".to_string(), 1.0);
    let err = NutritionVector::from_map(&map).unwrap_err();
    assert!(err.missing_fields.contains(&"alcohol_g"));
    assert!(err.suggestions.iter().any(|(a, c)| a == "alcohol_content" && *c == "alcohol_g"));
}
