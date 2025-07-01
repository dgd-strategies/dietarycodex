use dietarycodex::nutrition_vector::NutritionVector;
use std::fs;

#[test]
fn template_contains_required_fields() {
    let csv_text = fs::read_to_string("../data/template.csv").expect("read template.csv");
    let header_line = csv_text.lines().next().expect("empty csv");
    let headers: Vec<&str> = header_line.split(',').map(|s| s.trim()).collect();
    for field in NutritionVector::all_field_names() {
        assert!(
            headers.contains(&field),
            "template.csv missing required column {}",
            field
        );
    }
}
