use dietarycodex::eval::print_scores_as_json;
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
    let json = print_scores_as_json(&nv);
    println!("{}", json);
    Ok(())
}
