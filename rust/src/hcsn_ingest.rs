use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;

static HCSN_ALIASES_JSON: &str = include_str!("../../schema/hcsn_field_aliases.json");

static HCSN_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let raw: HashMap<String, String> = serde_json::from_str(HCSN_ALIASES_JSON)
        .expect("invalid hcsn_field_aliases.json");
    let mut map = HashMap::new();
    for (alias, canonical) in raw {
        let key: &'static str = Box::leak(alias.to_ascii_lowercase().into_boxed_str());
        let val: &'static str = Box::leak(canonical.into_boxed_str());
        map.insert(key, val);
    }
    map
});

pub type CanonicalField = &'static str;

pub fn is_hcsn_sheet(headers: &[String]) -> bool {
    headers
        .iter()
        .filter(|h| HCSN_MAP.contains_key(h.to_ascii_lowercase().as_str()))
        .count()
        >= 2
}

pub fn resolve_hcsn_headers(raw_headers: &[String]) -> HashMap<String, CanonicalField> {
    let mut map = HashMap::new();
    for h in raw_headers {
        if let Some(canon) = HCSN_MAP.get(h.to_ascii_lowercase().as_str()) {
            map.insert(h.clone(), *canon);
        }
    }
    map
}
