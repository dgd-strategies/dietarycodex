use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;

static ACS_ALIASES_JSON: &str = include_str!("../../schema/acs2020_field_aliases.json");

static ACS_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let raw: HashMap<String, String> = serde_json::from_str(ACS_ALIASES_JSON).expect("invalid acs2020_field_aliases.json");
    let mut map = HashMap::new();
    for (alias, canonical) in raw {
        let key: &'static str = Box::leak(alias.to_ascii_lowercase().into_boxed_str());
        let val: &'static str = Box::leak(canonical.into_boxed_str());
        map.insert(key, val);
    }
    map
});

pub type CanonicalField = &'static str;

pub fn is_acs2020_sheet(headers: &[String]) -> bool {
    headers
        .iter()
        .filter(|h| {
            let lower = h.to_ascii_lowercase();
            lower.ends_with("_acs2020") || ACS_MAP.contains_key(lower.as_str())
        })
        .count()
        >= 2
}

pub fn resolve_acs2020_headers(raw_headers: &[String]) -> HashMap<String, CanonicalField> {
    let mut map = HashMap::new();
    for h in raw_headers {
        let lower = h.to_ascii_lowercase();
        if let Some(canon) = ACS_MAP.get(lower.as_str()) {
            map.insert(h.clone(), *canon);
        } else if lower.ends_with("_acs2020") {
            let base = lower.trim_end_matches("_acs2020");
            if let Some(canon) = ACS_MAP.get(base) {
                map.insert(h.clone(), *canon);
            }
        }
    }
    map
}
