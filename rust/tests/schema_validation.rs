use dietarycodex::nutrition_vector::NutritionVector;
use std::collections::HashMap;
use serde_json;
use std::fs;

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
    assert!(err.missing_canonical_fields.contains(&"alcohol_g"));
    assert!(err.unmapped_aliases.contains(&"alcohol_content".to_string()));
}

#[test]
fn alias_map_covers_all_fields() {
    use std::fs;
    let data = fs::read_to_string("../schema/field_aliases.json").expect("read aliases");
    let raw: std::collections::HashMap<String, String> = serde_json::from_str(&data).expect("json");
    let mut counts: std::collections::HashMap<&str, usize> = std::collections::HashMap::new();
    for f in NutritionVector::all_field_names() {
        counts.insert(*f, 0);
    }
    for (_, canonical) in raw {
        if let Some(c) = counts.get_mut(canonical.as_str()) {
            *c += 1;
        } else {
            panic!("alias refers to unknown field {}", canonical);
        }
    }
    for (field, count) in counts {
        assert!(count > 0, "canonical field {} missing alias", field);
    }
}
