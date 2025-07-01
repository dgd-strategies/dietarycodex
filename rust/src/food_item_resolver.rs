use once_cell::sync::Lazy;
use serde::Deserialize;
use std::collections::HashMap;
use log::info;

static FOOD_JSON: &str = include_str!("../../schema/food_components.json");

#[derive(Debug, Deserialize)]
struct RawMap(HashMap<String, HashMap<String, f64>>);

pub struct FoodItemResolver {
    map: HashMap<&'static str, HashMap<&'static str, f64>>,
}

impl FoodItemResolver {
    fn load() -> Self {
        let raw: HashMap<String, HashMap<String, f64>> = serde_json::from_str(FOOD_JSON)
            .expect("invalid food_components.json");
        let mut map = HashMap::new();
        for (item, comps) in raw {
            let key: &'static str = Box::leak(item.to_ascii_lowercase().into_boxed_str());
            let mut inner = HashMap::new();
            for (nut, val) in comps {
                let nut_key: &'static str = Box::leak(nut.to_string().into_boxed_str());
                inner.insert(nut_key, val);
            }
            map.insert(key, inner);
        }
        FoodItemResolver { map }
    }

    pub fn resolve(&self, field: &str, amount: f64) -> Option<HashMap<&'static str, f64>> {
        let lower = field.to_ascii_lowercase();
        if lower.ends_with("_g") {
            let base = lower.trim_end_matches("_g");
            self.map.get(base).map(|comp| {
                comp.iter()
                    .map(|(k, v)| (*k, v * amount / 100.0))
                    .collect()
            })
        } else if lower.ends_with("_servings") {
            let base = lower.trim_end_matches("_servings");
            self.map.get(base).map(|comp| {
                comp.iter().map(|(k, v)| (*k, v * amount)).collect()
            })
        } else {
            None
        }
    }
}

pub static FOOD_RESOLVER: Lazy<FoodItemResolver> = Lazy::new(FoodItemResolver::load);

#[derive(Debug, Default, Clone, serde::Serialize, PartialEq)]
pub struct TranslationEntry {
    pub value: f64,
    pub source: Vec<String>,
}
