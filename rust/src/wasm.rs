use wasm_bindgen::prelude::*;
use serde::{Deserialize, Serialize};
use console_error_panic_hook;
use crate::nutrition_vector::NutritionVector;
use crate::eval::{evaluate_allow_partial, ScorerStatus};

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn score_json(json: &str) -> Result<JsValue, JsValue> {
    let records: Vec<NutritionVector> = serde_json::from_str(json)
        .map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut out: Vec<std::collections::BTreeMap<String, Option<f64>>> = Vec::new();
    for nv in records {
        let result = evaluate_allow_partial(&nv);
        let mut row = std::collections::BTreeMap::new();
        for name in result.ordered_names {
            let val = match result.scores.get(&name) {
                Some(ScorerStatus::Complete(v)) => Some(*v),
                _ => None,
            };
            row.insert(name.clone(), val);
        }
        out.push(row);
    }
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}
