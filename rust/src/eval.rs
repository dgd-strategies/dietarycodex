use crate::nutrition_vector::NutritionVector;
use crate::scores::all_scorers;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Debug, Serialize)]
pub struct ScoreResult {
    pub scores: HashMap<String, f64>,
    pub ordered_names: Vec<String>,
}

pub fn evaluate_all_scores(nv: &NutritionVector) -> ScoreResult {
    let calculators = all_scorers();
    let mut results = HashMap::new();
    let mut ordered = Vec::new();
    for calc in calculators {
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
    let result = evaluate_all_scores(nv);
    serde_json::to_string_pretty(&result).unwrap_or_else(|_| "{}".to_string())
}
