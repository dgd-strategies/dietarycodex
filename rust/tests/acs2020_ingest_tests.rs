use dietarycodex::acs2020_ingest::{is_acs2020_sheet, resolve_acs2020_headers};

#[test]
fn resolve_headers_maps_known_fields() {
    let headers = vec![
        "vegetable".to_string(),
        "fruit".to_string(),
        "kcal".to_string(),
    ];
    let map = resolve_acs2020_headers(&headers);
    assert_eq!(map.get("vegetable"), Some(&"VEG_SERV_ACS2020"));
    assert_eq!(map.get("fruit"), Some(&"FRT_SERV_ACS2020"));
    assert_eq!(map.get("kcal"), Some(&"TOTALKCAL_ACS2020"));
    assert!(is_acs2020_sheet(&headers));
}

#[test]
fn unknown_headers_not_mapped() {
    let headers = vec!["random".to_string(), "another".to_string()];
    let map = resolve_acs2020_headers(&headers);
    assert!(map.is_empty());
    assert!(!is_acs2020_sheet(&headers));
}
