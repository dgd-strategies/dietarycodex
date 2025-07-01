use crate::acs2020_ingest::{is_acs2020_sheet, resolve_acs2020_headers};
use crate::eval::evaluate_allow_partial;
use crate::hcsn_ingest::{is_hcsn_sheet, resolve_hcsn_headers};
use crate::nhanes_ingest::{is_nhanes_sheet, resolve_nhanes_headers};
use crate::nutrition_vector::{InputTrace, NutritionVector};
use crate::unmapped_monitor::UNMAPPED_MONITOR;
use console_error_panic_hook;
use serde_json;
use serde_json::Value;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[wasm_bindgen]
pub fn score_json(json: &str) -> Result<JsValue, JsValue> {
    let val: Value = serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let data_val = match &val {
        Value::Array(_) => val.clone(),
        Value::Object(map) => map
            .get("values")
            .cloned()
            .ok_or_else(|| JsValue::from_str("expected array or object with 'values'"))?,
        _ => return Err(JsValue::from_str("invalid json")),
    };
    let arr_str = serde_json::to_string(&data_val).unwrap();
    let mut records: Vec<std::collections::HashMap<String, Value>> =
        serde_json::from_str(&arr_str).map_err(|e| JsValue::from_str(&e.to_string()))?;

    if let Some(first) = records.first() {
        let headers: Vec<String> = first.keys().cloned().collect();
        if is_acs2020_sheet(&headers) {
            let map = resolve_acs2020_headers(&headers);
            for row in &mut records {
                let mut new_row = std::collections::HashMap::new();
                for (k, v) in row.iter() {
                    if let Some(&canon) = map.get(k) {
                        new_row.insert(canon.to_string(), v.clone());
                    } else {
                        new_row.insert(k.clone(), v.clone());
                    }
                }
                *row = new_row;
            }
        } else if is_hcsn_sheet(&headers) {
            let map = resolve_hcsn_headers(&headers);
            for row in &mut records {
                let mut new_row = std::collections::HashMap::new();
                for (k, v) in row.iter() {
                    if let Some(&canon) = map.get(k) {
                        new_row.insert(canon.to_string(), v.clone());
                    } else {
                        new_row.insert(k.clone(), v.clone());
                    }
                }
                *row = new_row;
            }
        } else if is_nhanes_sheet(&headers) {
            let map = resolve_nhanes_headers(&headers);
            for row in &mut records {
                let mut new_row = std::collections::HashMap::new();
                for (k, v) in row.iter() {
                    if let Some(&canon) = map.get(k) {
                        new_row.insert(canon.to_string(), v.clone());
                    } else {
                        new_row.insert(k.clone(), v.clone());
                    }
                }
                *row = new_row;
            }
        }
    }

    #[derive(serde::Serialize)]
    struct RowOutput {
        scores: std::collections::BTreeMap<String, Option<f64>>,
        validity: std::collections::BTreeMap<String, (bool, Option<String>)>,
        trace: InputTrace,
        errors: Vec<crate::eval::IndexError>,
    }

    let mut out: Vec<RowOutput> = Vec::new();
    let mut score_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();
    let mut missing_counts: std::collections::HashMap<&'static str, usize> =
        std::collections::HashMap::new();
    let mut alias_counts: std::collections::HashMap<String, usize> =
        std::collections::HashMap::new();

    for map in records {
        let (nv, trace) = NutritionVector::from_partial_map(&map);
        let mut result = evaluate_allow_partial(&nv);
        result.trace.aliases_applied = trace.aliases_applied.clone();
        result.trace.conflicting_aliases = trace.conflicting_aliases.clone();
        result.trace.translation_log = trace.translation_log.clone();

        for field in &result.trace.missing_fields {
            *missing_counts.entry(*field).or_insert(0) += 1;
        }
        for (alias, _) in &result.trace.aliases_applied {
            *alias_counts.entry(alias.clone()).or_insert(0) += 1;
        }

        let mut scores_map = std::collections::BTreeMap::new();
        let mut validity_map = std::collections::BTreeMap::new();
        for name in result.ordered_names {
            if let Some(info) = result.scores.get(&name) {
                if info.value.is_some() {
                    *score_counts.entry(name.clone()).or_insert(0) += 1;
                }
                scores_map.insert(name.clone(), info.value);
                validity_map.insert(name.clone(), (info.valid, info.explanation.clone()));
            }
        }

        out.push(RowOutput {
            scores: scores_map,
            validity: validity_map,
            trace: result.trace,
            errors: result.errors,
        });
    }

    let total_rows = out.len() as f64;
    #[derive(serde::Serialize)]
    struct Coverage {
        percent_scored: std::collections::BTreeMap<String, f64>,
        most_missing: Vec<(String, usize)>,
        alias_hits: std::collections::BTreeMap<String, usize>,
    }

    let mut pct = std::collections::BTreeMap::new();
    for (idx, count) in &score_counts {
        pct.insert(idx.clone(), (*count as f64) * 100.0 / total_rows.max(1.0));
    }
    let mut miss_vec: Vec<(String, usize)> = missing_counts
        .into_iter()
        .map(|(f, c)| (f.to_string(), c))
        .collect();
    miss_vec.sort_by(|a, b| b.1.cmp(&a.1));
    let alias_vec: std::collections::BTreeMap<String, usize> = alias_counts.into_iter().collect();

    #[derive(serde::Serialize)]
    struct Output {
        rows: Vec<RowOutput>,
        coverage: Coverage,
    }

    let coverage = Coverage {
        percent_scored: pct,
        most_missing: miss_vec,
        alias_hits: alias_vec,
    };
    let result = Output {
        rows: out,
        coverage,
    };
    serde_wasm_bindgen::to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn required_fields() -> JsValue {
    serde_wasm_bindgen::to_value(NutritionVector::all_field_names()).unwrap()
}

#[wasm_bindgen]
pub fn missing_fields(json: &str) -> Result<JsValue, JsValue> {
    let map: std::collections::HashMap<String, Value> =
        serde_json::from_str(json).map_err(|e| JsValue::from_str(&e.to_string()))?;
    let (nv, _) = NutritionVector::from_partial_map(&map);
    let missing = nv.missing_fields();
    serde_wasm_bindgen::to_value(&missing).map_err(|e| JsValue::from_str(&e.to_string()))
}

#[wasm_bindgen]
pub fn unmapped_log() -> JsValue {
    let snapshot = UNMAPPED_MONITOR.snapshot();
    serde_wasm_bindgen::to_value(&snapshot).unwrap()
}
