use crate::nutrition_vector::NutritionVector;
use crate::scores::all_scorers;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize, PartialEq)]
pub enum ScorerStatus {
    Complete(f64),
    Skipped { reason: String },
}

#[derive(Debug, Serialize)]
pub struct ScoreResult {
    pub scores: HashMap<String, ScorerStatus>,
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
        let name = calc.name().to_string();
        let required = calc.required_fields();
        let missing_fields: Vec<&str> = required
            .iter()
            .copied()
            .filter(|f| missing.contains(f))
            .collect();
        let status = if missing_fields.is_empty() {
            ScorerStatus::Complete(calc.evaluate(nv))
        } else {
            ScorerStatus::Skipped {
                reason: format!("missing fields: {}", missing_fields.join(", ")),
            }
        };
        ordered.push(name.clone());
        results.insert(name, status);
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
