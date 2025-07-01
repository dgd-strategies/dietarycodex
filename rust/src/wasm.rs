use crate::eval::{evaluate_all_scores, evaluate_allow_partial, ScoreInfo};
use crate::nutrition_vector::NutritionVector;
use console_error_panic_hook;
use serde_json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn score_json(json: &str) -> Result<JsValue, JsValue> {
    let records: Vec<NutritionVector> =
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let mut out: Vec<std::collections::BTreeMap<String, ScoreInfo>> = Vec::new();
    for nv in records {
        let result = evaluate_all_scores(&nv).map_err(|e| {
            JsValue::from_str(&serde_json::to_string(&e).unwrap_or_else(|_| "{}".to_string()))
        })?;
        let mut row = std::collections::BTreeMap::new();
        for name in result.ordered_names {
            if let Some(info) = result.scores.get(&name) {
                row.insert(name.clone(), info.clone());
            }
        }
        out.push(row);
    }
    serde_wasm_bindgen::to_value(&out).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn required_fields() -> JsValue {
    serde_wasm_bindgen::to_value(NutritionVector::all_field_names()).unwrap()
}

#[wasm_bindgen]
pub fn missing_fields(json: &str) -> Result<JsValue, JsValue> {
    let nv: NutritionVector =
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let missing = nv.missing_fields();
    serde_wasm_bindgen::to_value(&missing).map_err(|e| JsValue::from_str(&e.to_string()))
}
