use once_cell::sync::Lazy;
use serde_json;
use std::collections::HashMap;

use crate::nutrition_vector::{NutritionVector, SchemaError};

static HEADER_ALIASES_JSON: &str = include_str!("../../schema/header_aliases.json");

static HEADER_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let raw: HashMap<String, String> =
        serde_json::from_str(HEADER_ALIASES_JSON).expect("invalid header_aliases.json");
    let mut map = HashMap::new();
    for (alias, canonical) in raw {
        let key: &'static str = Box::leak(alias.to_ascii_lowercase().into_boxed_str());
        let val: &'static str = Box::leak(canonical.into_boxed_str());
        map.insert(key, val);
    }
    map
});

pub type CanonicalField = &'static str;

pub fn is_nhanes_sheet(headers: &[String]) -> bool {
    headers
        .iter()
        .filter(|h| HEADER_MAP.contains_key(h.to_ascii_lowercase().as_str()))
        .count()
        >= 2
}

pub fn resolve_nhanes_headers(raw_headers: &[String]) -> HashMap<String, CanonicalField> {
    let mut map = HashMap::new();
    for h in raw_headers {
        if let Some(canon) = HEADER_MAP.get(h.to_ascii_lowercase().as_str()) {
            map.insert(h.clone(), *canon);
        }
    }
    map
}

pub fn build_nutrition_vector_from_nhanes(
    row: &HashMap<String, f32>,
) -> Result<NutritionVector, SchemaError> {
    let mut temp: HashMap<&str, Vec<f64>> = HashMap::new();
    for (k, v) in row {
        if let Some(canon) = HEADER_MAP.get(k.to_ascii_lowercase().as_str()) {
            temp.entry(*canon).or_default().push(*v as f64);
        } else if crate::nutrition_vector::NutritionVector::all_field_names()
            .contains(&k.as_str())
        {
            let key: &'static str = Box::leak(k.as_str().to_string().into_boxed_str());
            temp.entry(key).or_default().push(*v as f64);
        }
    }

    let mut flat: HashMap<String, f64> = HashMap::new();
    for (canon, vals) in temp {
        let avg = vals.iter().sum::<f64>() / vals.len() as f64;
        flat.insert(canon.to_string(), avg);
    }
    NutritionVector::from_map(&flat)
}
