use crate::nutrition_vector::NutritionVector;
use crate::scores::{ahei::Ahei, DietScore};
use std::collections::HashMap;

pub fn evaluate_all_scores(nv: &NutritionVector) -> HashMap<String, f64> {
    let calculators: Vec<Box<dyn DietScore>> = vec![Box::new(Ahei)];
    let mut results = HashMap::new();
    for calc in calculators {
        let val = calc.score(nv);
        results.insert(calc.name().to_string(), val);
    }
    results
}
