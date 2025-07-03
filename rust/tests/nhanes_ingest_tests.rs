use dietarycodex::nhanes_ingest::{build_nutrition_vector_from_nhanes, is_nhanes_sheet, resolve_nhanes_headers};
use std::collections::HashMap;

#[test]
fn resolve_headers_maps_known_fields() {
    let headers = vec![
        "DR1TKCAL".to_string(),
        "DR2TKCAL".to_string(),
        "DR1TPROT".to_string(),
    ];
    let map = resolve_nhanes_headers(&headers);
    assert_eq!(map.get("DR1TKCAL"), Some(&"energy"));
    assert_eq!(map.get("DR1TPROT"), Some(&"protein"));
    assert!(is_nhanes_sheet(&headers));
}

#[test]
fn build_vector_averages_days() {
    let mut row: HashMap<String, f32> = HashMap::new();
    row.insert("DR1TKCAL".to_string(), 2000.0);
    row.insert("DR2TKCAL".to_string(), 1800.0);
    row.insert("DR1TPROT".to_string(), 60.0);
    row.insert("DR2TPROT".to_string(), 80.0);
    // Provide zeros for remaining canonical fields
    for field in dietarycodex::nutrition_vector::NutritionVector::all_field_names() {
        if *field != "energy" && *field != "protein" {
            row.insert(field.to_string(), 0.0);
        }
    }
    let nv = build_nutrition_vector_from_nhanes(&row).unwrap();
    assert_eq!(nv.energy, Some(1900.0));
    assert_eq!(nv.protein, Some(70.0));
}
