use crate::nutrition_vector::{NutritionVector, SchemaError};
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

pub fn evaluate_all_scores(nv: &NutritionVector) -> Result<ScoreResult, SchemaError> {
    let missing = nv.missing_fields();
    if !missing.is_empty() {
        return Err(SchemaError::new(missing, Vec::new()));
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
        let mut missing_fields: Vec<&str> = required
            .iter()
            .copied()
            .filter(|f| missing.contains(f))
            .collect();
        missing_fields.sort();
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
        Err(err) => serde_json::to_string_pretty(&err).unwrap_or_else(|_| "{}".to_string()),
    }
}

pub fn print_scores_as_json_allow_partial(nv: &NutritionVector) -> String {
    let result = evaluate_allow_partial(nv);
    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}

/// Collect skipped scores and their reasons sorted by name.
pub fn skipped_scores(result: &ScoreResult) -> Vec<(String, String)> {
    let mut skipped: Vec<(String, String)> = result
        .ordered_names
        .iter()
        .filter_map(|name| {
            result.scores.get(name).and_then(|s| match s {
                ScorerStatus::Skipped { reason } => Some((name.clone(), reason.clone())),
                _ => None,
            })
        })
        .collect();
    skipped.sort_by(|a, b| a.0.cmp(&b.0));
    skipped
}

/// Format skipped scores for human-readable output.
pub fn format_skipped_scores(result: &ScoreResult) -> Option<String> {
    let skipped = skipped_scores(result);
    if skipped.is_empty() {
        return None;
    }
    let mut out = String::from("Skipped scores:\n");
    for (name, reason) in skipped {
        out.push_str(&format!("  {}: {}\n", name, reason));
    }
    Some(out)
}
