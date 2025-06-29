use dietarycodex::eval::evaluate_all_scores;
use dietarycodex::nutrition_vector::NutritionVector;
use std::fs;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <fdc_json>", args[0]);
        std::process::exit(1);
    }
    let data = fs::read_to_string(&args[1])?;
    let nv = NutritionVector::from_fdc_json(&data)?;
    let scores = evaluate_all_scores(&nv);
    println!("{}", serde_json::to_string_pretty(&scores)?);
    Ok(())
}
