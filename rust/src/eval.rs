use crate::nutrition_vector::NutritionVector;
use crate::scores::all_scorers;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ScoreResult {
    pub scores: HashMap<String, f64>,
    pub ordered_names: Vec<String>,
}


pub fn evaluate_all_scores(nv: &NutritionVector) -> Result<ScoreResult, Vec<&'static str>> {
    let missing = nv.missing_fields();
    if !missing.is_empty() {
        return Err(missing);
    }
    Ok(evaluate_allow_partial(nv))
}

pub fn evaluate_allow_partial(nv: &NutritionVector) -> ScoreResult {
    let calculators = all_scorers();
    let mut results = HashMap::new();
    let mut ordered = Vec::new();
    let missing = nv.missing_fields();
    for calc in calculators {
        if calc.required_fields().iter().any(|f| missing.contains(f)) {
            continue;
        }
        let name = calc.name().to_string();
        let val = calc.evaluate(nv);
        ordered.push(name.clone());
        results.insert(name, val);
    }
    ScoreResult {
        scores: results,
        ordered_names: ordered,
    }
}

pub fn print_scores_as_json(nv: &NutritionVector) -> String {
    match evaluate_all_scores(nv) {
        Ok(result) => serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string()),
        Err(missing) => serde_json::to_string_pretty(&missing).unwrap_or_else(|_| "{}".to_string()),
    }
}

pub fn print_scores_as_json_allow_partial(nv: &NutritionVector) -> String {
    let result = evaluate_allow_partial(nv);
    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}
