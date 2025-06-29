use dietarycodex::eval::{print_scores_as_json, print_scores_as_json_allow_partial};
use dietarycodex::nutrition_vector::NutritionVector;
use std::fs;
use std::env;

fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <fdc_json> [--allow-partial]", args[0]);
        std::process::exit(1);
    }
    let mut allow_partial = false;
    let mut file = String::new();
    for arg in args.iter().skip(1) {
        if arg == "--allow-partial" {
            allow_partial = true;
        } else {
            file = arg.clone();
        }
    }
    if file.is_empty() {
        eprintln!("Usage: {} <fdc_json> [--allow-partial]", args[0]);
        std::process::exit(1);
    }
    let data = fs::read_to_string(&file)?;
    let nv = NutritionVector::from_fdc_json(&data)?;
    let json = if allow_partial {
        print_scores_as_json_allow_partial(&nv)
    } else {
        print_scores_as_json(&nv)
    };
    println!("{}", json);
    Ok(())
}
