use dietarycodex::hcsn_ingest::{is_hcsn_sheet, resolve_hcsn_headers};

#[test]
fn resolve_headers_maps_known_fields() {
    let headers = vec![
        "leafy_green_veg_servings".to_string(),
        "butter_servings".to_string(),
        "whole_grain_g".to_string(),
    ];
    let map = resolve_hcsn_headers(&headers);
    assert_eq!(map.get("leafy_green_veg_servings"), Some(&"vegetables"));
    assert_eq!(map.get("butter_servings"), Some(&"butter"));
    assert_eq!(map.get("whole_grain_g"), Some(&"whole_grains"));
    assert!(is_hcsn_sheet(&headers));
}

#[test]
fn unknown_headers_not_mapped() {
    let headers = vec!["random".to_string(), "another".to_string()];
    let map = resolve_hcsn_headers(&headers);
    assert!(map.is_empty());
    assert!(!is_hcsn_sheet(&headers));
}
