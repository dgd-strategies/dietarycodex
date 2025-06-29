use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NutritionVector {
    pub energy_kcal: f64,
    pub fat_g: f64,
    pub saturated_fat_g: f64,
    pub carbs_g: f64,
    pub fiber_g: f64,
    pub sugar_g: f64,
    pub protein_g: f64,
    pub sodium_mg: f64,
    pub calcium_mg: f64,
    pub iron_mg: f64,
    pub vitamin_c_mg: f64,
    pub total_fruits_g: f64,
    pub vegetables_g: f64,
    pub whole_grains_g: f64,
    pub refined_grains_g: f64,
    pub legumes_g: f64,
    pub fish_g: f64,
    pub red_meat_g: f64,
    pub mono_fat_g: f64,
    pub berries_g: f64,
    pub cheese_g: f64,
    pub butter_g: f64,
    pub poultry_g: f64,
    pub fast_food_g: f64,
    pub nuts_g: f64,
    // Additional nutrients for indices like DII
    pub omega3_g: f64,
    pub vitamin_a_mcg: f64,
    pub vitamin_e_mg: f64,
    pub zinc_mg: f64,
    pub selenium_mcg: f64,
    pub magnesium_mg: f64,
    pub trans_fat_g: f64,
    pub alcohol_g: f64,
}

impl NutritionVector {
    pub fn from_fdc_json(data: &str) -> anyhow::Result<Self> {
        let v: Value = serde_json::from_str(data)?;
        let mut nv = NutritionVector::default();
        if let Some(items) = v.get("foodNutrients").and_then(|v| v.as_array()) {
            let mut map: HashMap<&str, f64> = HashMap::new();
            for item in items {
                if let (Some(nutrient), Some(amount)) = (item.get("nutrient"), item.get("amount")) {
                    let name = nutrient.get("name").and_then(|n| n.as_str()).unwrap_or("");
                    let unit = nutrient
                        .get("unitName")
                        .and_then(|n| n.as_str())
                        .unwrap_or("");
                    let amount = amount.as_f64().unwrap_or(0.0);
                    map.insert(
                        name,
                        match unit.to_ascii_lowercase().as_str() {
                            "mg" => amount / 1000.0,
                            _ => amount,
                        },
                    );
                }
            }
            nv.energy_kcal = *map.get("Energy").unwrap_or(&0.0);
            nv.fat_g = *map.get("Total lipid (fat)").unwrap_or(&0.0);
            nv.saturated_fat_g = *map.get("Fatty acids, total saturated").unwrap_or(&0.0);
            nv.carbs_g = *map.get("Carbohydrate, by difference").unwrap_or(&0.0);
            nv.fiber_g = *map.get("Fiber, total dietary").unwrap_or(&0.0);
            nv.sugar_g = *map.get("Sugars, total including NLEA").unwrap_or(&0.0);
            nv.protein_g = *map.get("Protein").unwrap_or(&0.0);
            nv.sodium_mg = map.get("Sodium, Na").unwrap_or(&0.0) * 1000.0;
            nv.calcium_mg = map.get("Calcium, Ca").unwrap_or(&0.0) * 1000.0;
            nv.iron_mg = map.get("Iron, Fe").unwrap_or(&0.0) * 1000.0;
            nv.vitamin_c_mg = map.get("Vitamin C, total ascorbic acid").unwrap_or(&0.0) * 1000.0;
            nv.omega3_g = *map.get("Fatty acids, total polyunsaturated").unwrap_or(&0.0);
            nv.vitamin_a_mcg = *map.get("Vitamin A, RAE").unwrap_or(&0.0);
            nv.vitamin_e_mg = map.get("Vitamin E (alpha-tocopherol)").unwrap_or(&0.0) * 1000.0;
            nv.zinc_mg = map.get("Zinc, Zn").unwrap_or(&0.0) * 1000.0;
            nv.selenium_mcg = *map.get("Selenium, Se").unwrap_or(&0.0);
            nv.magnesium_mg = map.get("Magnesium, Mg").unwrap_or(&0.0) * 1000.0;
            nv.trans_fat_g = *map.get("Fatty acids, total trans").unwrap_or(&0.0);
            nv.alcohol_g = *map.get("Alcohol, ethyl").unwrap_or(&0.0);
        }
        Ok(nv)
    }
}
