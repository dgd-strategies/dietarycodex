use once_cell::sync::Lazy;
use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;
use crate::food_item_resolver::{FOOD_RESOLVER, TranslationEntry};
use crate::unmapped_monitor::UNMAPPED_MONITOR;
use log::info;
#[cfg(feature = "hot_reload_aliases")]
use std::sync::RwLock;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NutritionVector {
    pub energy_kcal: Option<f64>,
    pub fat_g: Option<f64>,
    pub saturated_fat_g: Option<f64>,
    pub carbs_g: Option<f64>,
    pub fiber_g: Option<f64>,
    pub sugar_g: Option<f64>,
    pub protein_g: Option<f64>,
    pub sodium_mg: Option<f64>,
    pub calcium_mg: Option<f64>,
    pub iron_mg: Option<f64>,
    pub vitamin_c_mg: Option<f64>,
    pub total_fruits_g: Option<f64>,
    pub vegetables_g: Option<f64>,
    pub whole_grains_g: Option<f64>,
    pub refined_grains_g: Option<f64>,
    pub legumes_g: Option<f64>,
    pub fish_g: Option<f64>,
    pub red_meat_g: Option<f64>,
    pub mono_fat_g: Option<f64>,
    pub berries_g: Option<f64>,
    pub cheese_g: Option<f64>,
    pub butter_g: Option<f64>,
    pub poultry_g: Option<f64>,
    pub fast_food_g: Option<f64>,
    pub nuts_g: Option<f64>,
    // Additional nutrients for indices like DII
    pub omega3_g: Option<f64>,
    pub vitamin_a_mcg: Option<f64>,
    pub vitamin_e_mg: Option<f64>,
    pub zinc_mg: Option<f64>,
    pub selenium_mcg: Option<f64>,
    pub magnesium_mg: Option<f64>,
    pub trans_fat_g: Option<f64>,
    #[serde(alias = "ALCOHOL", alias = "alcohol_intake")]
    pub alcohol_g: Option<f64>,
}

static TEMPLATE_CSV: &str = include_str!("../../data/template.csv");
/// Canonical field set derived from scoring contracts
static CANONICAL_SET: &[&str] = &[
    "alcohol_g",
    "berries_g",
    "butter_g",
    "calcium_mg",
    "carbs_g",
    "cheese_g",
    "energy_kcal",
    "fast_food_g",
    "fat_g",
    "fiber_g",
    "fish_g",
    "iron_mg",
    "legumes_g",
    "magnesium_mg",
    "mono_fat_g",
    "nuts_g",
    "omega3_g",
    "poultry_g",
    "protein_g",
    "red_meat_g",
    "refined_grains_g",
    "saturated_fat_g",
    "selenium_mcg",
    "sodium_mg",
    "sugar_g",
    "total_fruits_g",
    "trans_fat_g",
    "vegetables_g",
    "vitamin_a_mcg",
    "vitamin_c_mg",
    "vitamin_e_mg",
    "whole_grains_g",
    "zinc_mg",
];

static ALL_FIELD_NAMES: Lazy<Vec<&'static str>> = Lazy::new(|| {
    let header_line = TEMPLATE_CSV.lines().next().expect("template.csv empty");
    let headers: Vec<&str> = header_line.split(',').map(|s| s.trim()).collect();
    let mut fields = Vec::new();
    for h in headers {
        if CANONICAL_SET.contains(&h) {
            fields.push(Box::leak(h.to_string().into_boxed_str()) as &'static str);
        }
    }
    for required in CANONICAL_SET {
        assert!(fields.contains(required), "template.csv missing {required}");
    }
    fields
});

static FIELD_ORDER_MAP: Lazy<HashMap<&'static str, usize>> = Lazy::new(|| {
    let mut map = HashMap::new();
    for (idx, field) in ALL_FIELD_NAMES.iter().enumerate() {
        map.insert(*field, idx);
    }
    map
});

static FIELD_ALIASES_JSON: &str = include_str!("../../schema/field_aliases.json");

#[cfg(not(feature = "hot_reload_aliases"))]
static FIELD_ALIAS_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let raw: HashMap<String, String> =
        serde_json::from_str(FIELD_ALIASES_JSON).expect("invalid field_aliases.json");
    let mut map: HashMap<&'static str, &'static str> = HashMap::new();
    for &field in ALL_FIELD_NAMES.iter() {
        map.insert(field, field);
    }
    for (alias, canonical) in raw {
        let canonical_static = ALL_FIELD_NAMES
            .iter()
            .copied()
            .find(|f| *f == canonical)
            .unwrap_or_else(|| panic!("alias {} refers to unknown field {}", alias, canonical));
        map.insert(
            Box::leak(alias.to_ascii_lowercase().into_boxed_str()),
            canonical_static,
        );
    }
    map
});

#[cfg(feature = "hot_reload_aliases")]
static FIELD_ALIAS_MAP: Lazy<RwLock<HashMap<String, &'static str>>> =
    Lazy::new(|| RwLock::new(load_alias_map()));

#[cfg(feature = "hot_reload_aliases")]
fn load_alias_map() -> HashMap<String, &'static str> {
    use std::fs;
    use std::path::Path;

    let path = Path::new(env!("CARGO_MANIFEST_DIR")).join("../schema/field_aliases.json");
    let data = fs::read_to_string(path).expect("read field_aliases.json");
    let raw: HashMap<String, String> =
        serde_json::from_str(&data).expect("invalid field_aliases.json");
    let mut map: HashMap<String, &'static str> = HashMap::new();
    for &field in ALL_FIELD_NAMES.iter() {
        map.insert(field.to_string(), field);
    }
    for (alias, canonical) in raw {
        let canonical_static = ALL_FIELD_NAMES
            .iter()
            .copied()
            .find(|f| *f == canonical)
            .unwrap_or_else(|| panic!("alias {} refers to unknown field {}", alias, canonical));
        map.insert(alias.to_ascii_lowercase(), canonical_static);
    }
    map
}

#[cfg(feature = "hot_reload_aliases")]
pub fn reload_aliases() {
    let new_map = load_alias_map();
    let mut guard = FIELD_ALIAS_MAP.write().unwrap();
    *guard = new_map;
}

#[cfg(not(feature = "hot_reload_aliases"))]
fn canonical_field(name: &str) -> Option<&'static str> {
    FIELD_ALIAS_MAP
        .get(&name.to_ascii_lowercase() as &str)
        .copied()
}

#[cfg(feature = "hot_reload_aliases")]
fn canonical_field(name: &str) -> Option<&'static str> {
    FIELD_ALIAS_MAP
        .read()
        .unwrap()
        .get(&name.to_ascii_lowercase())
        .copied()
}

fn guess_canonical(name: &str) -> Option<&'static str> {
    let lower = name.to_ascii_lowercase();
    ALL_FIELD_NAMES
        .iter()
        .copied()
        .find(|f| lower.contains(f.trim_end_matches("_g")))
}

use serde::Serialize;

#[derive(Debug, PartialEq, Serialize)]
pub struct SchemaError {
    pub missing_canonical_fields: Vec<&'static str>,
    pub unmapped_aliases: Vec<String>,
    pub conflicting_aliases: Vec<(String, &'static str)>,
    pub index_dependencies: HashMap<&'static str, Vec<&'static str>>,
}

#[derive(Debug, Default, Clone, Serialize, PartialEq)]
pub struct InputTrace {
    pub used_fields: Vec<&'static str>,
    pub missing_fields: Vec<&'static str>,
    pub aliases_applied: Vec<(String, &'static str)>,
    pub conflicting_aliases: Vec<(String, &'static str)>,
    pub translation_log: HashMap<&'static str, TranslationEntry>,
}

impl InputTrace {
    pub fn from_nv(nv: &NutritionVector) -> Self {
        let missing = nv.missing_fields();
        let used: Vec<&'static str> = NutritionVector::all_field_names()
            .iter()
            .copied()
            .filter(|f| !missing.contains(f))
            .collect();
        InputTrace {
            used_fields: used,
            missing_fields: missing,
            aliases_applied: Vec::new(),
            conflicting_aliases: Vec::new(),
            translation_log: HashMap::new(),
        }
    }
}

impl SchemaError {
    pub fn new(
        missing: Vec<&'static str>,
        unmapped: Vec<String>,
        conflicts: Vec<(String, &'static str)>,
    ) -> Self {
        use crate::scores::registry::all_score_metadata;
        let mut deps: HashMap<&'static str, Vec<&'static str>> = HashMap::new();
        let metas = all_score_metadata();
        for field in &missing {
            let indices: Vec<&'static str> = metas
                .iter()
                .filter_map(|m| {
                    if m.required_fields.contains(field) {
                        Some(m.name)
                    } else {
                        None
                    }
                })
                .collect();
            deps.insert(*field, indices);
        }
        SchemaError {
            missing_canonical_fields: missing,
            unmapped_aliases: unmapped,
            conflicting_aliases: conflicts,
            index_dependencies: deps,
        }
    }
}

impl NutritionVector {
    pub fn from_fdc_json(data: &str) -> anyhow::Result<Self> {
        let v: Value = serde_json::from_str(data)?;
        let mut nv = NutritionVector::default();
        if let Some(items) = v.get("foodNutrients").and_then(|v| v.as_array()) {
            let mut map: HashMap<&str, f64> = HashMap::new();
            for item in items {
                if let (Some(nutrient), Some(amount)) = (item.get("nutrient"), item.get("amount")) {
                    let name = nutrient.get("name").and_then(|n| n.as_str()).unwrap_or("");
                    let unit = nutrient
                        .get("unitName")
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let amount = amount.as_f64().unwrap_or(0.0);
                    map.insert(
                        name,
                        match unit.to_ascii_lowercase().as_str() {
                            "mg" => amount / 1000.0,
                            _ => amount,
                        },
                    );
                }
            }
            nv.energy_kcal = map.get("Energy").copied();
            nv.fat_g = map.get("Total lipid (fat)").copied();
            nv.saturated_fat_g = map.get("Fatty acids, total saturated").copied();
            nv.carbs_g = map.get("Carbohydrate, by difference").copied();
            nv.fiber_g = map.get("Fiber, total dietary").copied();
            nv.sugar_g = map.get("Sugars, total including NLEA").copied();
            nv.protein_g = map.get("Protein").copied();
            nv.sodium_mg = map.get("Sodium, Na").map(|v| v * 1000.0);
            nv.calcium_mg = map.get("Calcium, Ca").map(|v| v * 1000.0);
            nv.iron_mg = map.get("Iron, Fe").map(|v| v * 1000.0);
            nv.vitamin_c_mg = map
                .get("Vitamin C, total ascorbic acid")
                .map(|v| v * 1000.0);
            nv.omega3_g = map.get("Fatty acids, total polyunsaturated").copied();
            nv.vitamin_a_mcg = map.get("Vitamin A, RAE").copied();
            nv.vitamin_e_mg = map.get("Vitamin E (alpha-tocopherol)").map(|v| v * 1000.0);
            nv.zinc_mg = map.get("Zinc, Zn").map(|v| v * 1000.0);
            nv.selenium_mcg = map.get("Selenium, Se").copied();
            nv.magnesium_mg = map.get("Magnesium, Mg").map(|v| v * 1000.0);
            nv.trans_fat_g = map.get("Fatty acids, total trans").copied();
            nv.alcohol_g = map.get("Alcohol, ethyl").copied();
        }
        Ok(nv)
    }

    pub fn from_map(data: &HashMap<String, f64>) -> Result<Self, SchemaError> {
        let mut obj = serde_json::Map::new();
        let mut unmapped = Vec::new();
        let mut conflicts = Vec::new();
        let mut chosen: HashMap<&str, String> = HashMap::new();
        let mut food_contrib: HashMap<&'static str, f64> = HashMap::new();

        let mut items: Vec<(&String, &f64)> = data.iter().collect();
        items.sort_by_key(|(k, _)| {
            canonical_field(k)
                .and_then(|c| FIELD_ORDER_MAP.get(c))
                .copied()
                .unwrap_or(usize::MAX)
        });

        for (k, v) in items {
            match canonical_field(k) {
                Some(canon) => {
                    if obj.contains_key(canon) {
                        let current = chosen.get(canon).cloned().unwrap_or_else(|| canon.to_string());
                        if k != canon {
                            conflicts.push((k.clone(), canon));
                        } else if current != canon {
                            conflicts.push((current.clone(), canon));
                        }
                        let prefer_new = k == canon && current != canon;
                        if prefer_new {
                            obj.insert(canon.to_string(), serde_json::json!(v));
                            chosen.insert(canon, k.clone());
                        }
                    } else {
                        obj.insert(canon.to_string(), serde_json::json!(v));
                        chosen.insert(canon, k.clone());
                    }
                }
                None => {
                    if let Some(map) = FOOD_RESOLVER.resolve(k, *v) {
                        for (field, val) in map {
                            *food_contrib.entry(field).or_insert(0.0) += val;
                        }
                    } else {
                        info!("TODO: map food field {}", k);
                        UNMAPPED_MONITOR.log(k, Some(*v));
                        unmapped.push(k.clone());
                    }
                }
            }
        }

        for (field, val) in food_contrib {
            if let Some(existing) = obj.get_mut(field) {
                if let Some(num) = existing.as_f64() {
                    *existing = serde_json::json!(num + val);
                }
            } else {
                obj.insert(field.to_string(), serde_json::json!(val));
            }
        }

        let nv: NutritionVector = serde_json::from_value(Value::Object(obj)).unwrap_or_default();
        let missing = nv.missing_fields();
        if !missing.is_empty() || !unmapped.is_empty() || !conflicts.is_empty() {
            return Err(SchemaError::new(missing, unmapped, conflicts));
        }
        Ok(nv)
    }

    pub fn from_partial_map(data: &HashMap<String, Value>) -> (Self, InputTrace) {
        let mut obj = serde_json::Map::new();
        let mut aliases = Vec::new();
        let mut conflicts = Vec::new();
        let mut chosen: HashMap<&str, String> = HashMap::new();
        let mut translation: HashMap<&'static str, TranslationEntry> = HashMap::new();
        let mut food_contrib: HashMap<&'static str, f64> = HashMap::new();

        let mut items: Vec<(&String, &Value)> = data.iter().collect();
        items.sort_by_key(|(k, _)| {
            canonical_field(k)
                .and_then(|c| FIELD_ORDER_MAP.get(c))
                .copied()
                .unwrap_or(usize::MAX)
        });

        for (k, v) in items {
            if let Some(canon) = canonical_field(k) {
                if obj.contains_key(canon) {
                    let current = chosen.get(canon).cloned().unwrap_or_else(|| canon.to_string());
                    if k != canon {
                        conflicts.push((k.clone(), canon));
                    } else if current != canon {
                        conflicts.push((current.clone(), canon));
                    }
                    let prefer_new = k == canon && current != canon;
                    if prefer_new {
                        obj.insert(canon.to_string(), v.clone());
                        chosen.insert(canon, k.clone());
                    }
                } else {
                    if canon != k.as_str() {
                        aliases.push((k.clone(), canon));
                    }
                    obj.insert(canon.to_string(), v.clone());
                    chosen.insert(canon, k.clone());
                }
            } else if let Some(num) = v.as_f64() {
                if let Some(map) = FOOD_RESOLVER.resolve(k, num) {
                    for (field, val) in map {
                        *food_contrib.entry(field).or_insert(0.0) += val;
                        let entry = translation.entry(field).or_default();
                        entry.value += val;
                        entry.source.push(k.clone());
                    }
                } else {
                    info!("TODO: map food field {}", k);
                    UNMAPPED_MONITOR.log(k, Some(num));
                }
            }
        }
        for (field, val) in &food_contrib {
            if let Some(existing) = obj.get_mut(*field) {
                if let Some(num) = existing.as_f64() {
                    *existing = serde_json::json!(num + val);
                }
            } else {
                obj.insert(field.to_string(), serde_json::json!(val));
            }
        }
        let nv: NutritionVector = serde_json::from_value(Value::Object(obj)).unwrap_or_default();
        let missing = nv.missing_fields();
        let used: Vec<&'static str> = NutritionVector::all_field_names()
            .iter()
            .copied()
            .filter(|f| !missing.contains(f))
            .collect();
        (
            nv,
            InputTrace {
                used_fields: used,
                missing_fields: missing,
                aliases_applied: aliases,
                conflicting_aliases: conflicts,
                translation_log: translation,
            },
        )
    }

    pub fn missing_fields(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.energy_kcal.is_none() {
            missing.push("energy_kcal");
        }
        if self.fat_g.is_none() {
            missing.push("fat_g");
        }
        if self.saturated_fat_g.is_none() {
            missing.push("saturated_fat_g");
        }
        if self.carbs_g.is_none() {
            missing.push("carbs_g");
        }
        if self.fiber_g.is_none() {
            missing.push("fiber_g");
        }
        if self.sugar_g.is_none() {
            missing.push("sugar_g");
        }
        if self.protein_g.is_none() {
            missing.push("protein_g");
        }
        if self.sodium_mg.is_none() {
            missing.push("sodium_mg");
        }
        if self.calcium_mg.is_none() {
            missing.push("calcium_mg");
        }
        if self.iron_mg.is_none() {
            missing.push("iron_mg");
        }
        if self.vitamin_c_mg.is_none() {
            missing.push("vitamin_c_mg");
        }
        if self.total_fruits_g.is_none() {
            missing.push("total_fruits_g");
        }
        if self.vegetables_g.is_none() {
            missing.push("vegetables_g");
        }
        if self.whole_grains_g.is_none() {
            missing.push("whole_grains_g");
        }
        if self.refined_grains_g.is_none() {
            missing.push("refined_grains_g");
        }
        if self.legumes_g.is_none() {
            missing.push("legumes_g");
        }
        if self.fish_g.is_none() {
            missing.push("fish_g");
        }
        if self.red_meat_g.is_none() {
            missing.push("red_meat_g");
        }
        if self.mono_fat_g.is_none() {
            missing.push("mono_fat_g");
        }
        if self.berries_g.is_none() {
            missing.push("berries_g");
        }
        if self.cheese_g.is_none() {
            missing.push("cheese_g");
        }
        if self.butter_g.is_none() {
            missing.push("butter_g");
        }
        if self.poultry_g.is_none() {
            missing.push("poultry_g");
        }
        if self.fast_food_g.is_none() {
            missing.push("fast_food_g");
        }
        if self.nuts_g.is_none() {
            missing.push("nuts_g");
        }
        if self.omega3_g.is_none() {
            missing.push("omega3_g");
        }
        if self.vitamin_a_mcg.is_none() {
            missing.push("vitamin_a_mcg");
        }
        if self.vitamin_e_mg.is_none() {
            missing.push("vitamin_e_mg");
        }
        if self.zinc_mg.is_none() {
            missing.push("zinc_mg");
        }
        if self.selenium_mcg.is_none() {
            missing.push("selenium_mcg");
        }
        if self.magnesium_mg.is_none() {
            missing.push("magnesium_mg");
        }
        if self.trans_fat_g.is_none() {
            missing.push("trans_fat_g");
        }
        if self.alcohol_g.is_none() {
            missing.push("alcohol_g");
        }
        missing
    }

    pub fn all_field_names() -> &'static [&'static str] {
        ALL_FIELD_NAMES.as_slice()
    }
}
