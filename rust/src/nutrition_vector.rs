use serde::Deserialize;
use serde_json::Value;
use std::collections::HashMap;

#[derive(Debug, Default, Clone, Deserialize)]
pub struct NutritionVector {
    pub energy_kcal: Option<f64>,
    pub fat_g: Option<f64>,
    pub saturated_fat_g: Option<f64>,
    pub carbs_g: Option<f64>,
    pub fiber_g: Option<f64>,
    pub sugar_g: Option<f64>,
    pub protein_g: Option<f64>,
    pub sodium_mg: Option<f64>,
    pub calcium_mg: Option<f64>,
    pub iron_mg: Option<f64>,
    pub vitamin_c_mg: Option<f64>,
    pub total_fruits_g: Option<f64>,
    pub vegetables_g: Option<f64>,
    pub whole_grains_g: Option<f64>,
    pub refined_grains_g: Option<f64>,
    pub legumes_g: Option<f64>,
    pub fish_g: Option<f64>,
    pub red_meat_g: Option<f64>,
    pub mono_fat_g: Option<f64>,
    pub berries_g: Option<f64>,
    pub cheese_g: Option<f64>,
    pub butter_g: Option<f64>,
    pub poultry_g: Option<f64>,
    pub fast_food_g: Option<f64>,
    pub nuts_g: Option<f64>,
    // Additional nutrients for indices like DII
    pub omega3_g: Option<f64>,
    pub vitamin_a_mcg: Option<f64>,
    pub vitamin_e_mg: Option<f64>,
    pub zinc_mg: Option<f64>,
    pub selenium_mcg: Option<f64>,
    pub magnesium_mg: Option<f64>,
    pub trans_fat_g: Option<f64>,
    #[serde(alias = "ALCOHOL", alias = "alcohol_intake")]
    pub alcohol_g: Option<f64>,
}

static ALL_FIELD_NAMES: &[&str] = &[
    "alcohol_g",
    "berries_g",
    "butter_g",
    "calcium_mg",
    "carbs_g",
    "cheese_g",
    "energy_kcal",
    "fast_food_g",
    "fat_g",
    "fiber_g",
    "fish_g",
    "iron_mg",
    "legumes_g",
    "magnesium_mg",
    "mono_fat_g",
    "nuts_g",
    "omega3_g",
    "poultry_g",
    "protein_g",
    "red_meat_g",
    "refined_grains_g",
    "saturated_fat_g",
    "selenium_mcg",
    "sodium_mg",
    "sugar_g",
    "total_fruits_g",
    "trans_fat_g",
    "vegetables_g",
    "vitamin_a_mcg",
    "vitamin_c_mg",
    "vitamin_e_mg",
    "whole_grains_g",
    "zinc_mg",
];

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
            nv.energy_kcal = map.get("Energy").copied();
            nv.fat_g = map.get("Total lipid (fat)").copied();
            nv.saturated_fat_g = map.get("Fatty acids, total saturated").copied();
            nv.carbs_g = map.get("Carbohydrate, by difference").copied();
            nv.fiber_g = map.get("Fiber, total dietary").copied();
            nv.sugar_g = map.get("Sugars, total including NLEA").copied();
            nv.protein_g = map.get("Protein").copied();
            nv.sodium_mg = map.get("Sodium, Na").map(|v| v * 1000.0);
            nv.calcium_mg = map.get("Calcium, Ca").map(|v| v * 1000.0);
            nv.iron_mg = map.get("Iron, Fe").map(|v| v * 1000.0);
            nv.vitamin_c_mg = map
                .get("Vitamin C, total ascorbic acid")
                .map(|v| v * 1000.0);
            nv.omega3_g = map.get("Fatty acids, total polyunsaturated").copied();
            nv.vitamin_a_mcg = map.get("Vitamin A, RAE").copied();
            nv.vitamin_e_mg = map.get("Vitamin E (alpha-tocopherol)").map(|v| v * 1000.0);
            nv.zinc_mg = map.get("Zinc, Zn").map(|v| v * 1000.0);
            nv.selenium_mcg = map.get("Selenium, Se").copied();
            nv.magnesium_mg = map.get("Magnesium, Mg").map(|v| v * 1000.0);
            nv.trans_fat_g = map.get("Fatty acids, total trans").copied();
            nv.alcohol_g = map.get("Alcohol, ethyl").copied();
        }
        Ok(nv)
    }

    pub fn missing_fields(&self) -> Vec<&'static str> {
        let mut missing = Vec::new();
        if self.energy_kcal.is_none() {
            missing.push("energy_kcal");
        }
        if self.fat_g.is_none() {
            missing.push("fat_g");
        }
        if self.saturated_fat_g.is_none() {
            missing.push("saturated_fat_g");
        }
        if self.carbs_g.is_none() {
            missing.push("carbs_g");
        }
        if self.fiber_g.is_none() {
            missing.push("fiber_g");
        }
        if self.sugar_g.is_none() {
            missing.push("sugar_g");
        }
        if self.protein_g.is_none() {
            missing.push("protein_g");
        }
        if self.sodium_mg.is_none() {
            missing.push("sodium_mg");
        }
        if self.calcium_mg.is_none() {
            missing.push("calcium_mg");
        }
        if self.iron_mg.is_none() {
            missing.push("iron_mg");
        }
        if self.vitamin_c_mg.is_none() {
            missing.push("vitamin_c_mg");
        }
        if self.total_fruits_g.is_none() {
            missing.push("total_fruits_g");
        }
        if self.vegetables_g.is_none() {
            missing.push("vegetables_g");
        }
        if self.whole_grains_g.is_none() {
            missing.push("whole_grains_g");
        }
        if self.refined_grains_g.is_none() {
            missing.push("refined_grains_g");
        }
        if self.legumes_g.is_none() {
            missing.push("legumes_g");
        }
        if self.fish_g.is_none() {
            missing.push("fish_g");
        }
        if self.red_meat_g.is_none() {
            missing.push("red_meat_g");
        }
        if self.mono_fat_g.is_none() {
            missing.push("mono_fat_g");
        }
        if self.berries_g.is_none() {
            missing.push("berries_g");
        }
        if self.cheese_g.is_none() {
            missing.push("cheese_g");
        }
        if self.butter_g.is_none() {
            missing.push("butter_g");
        }
        if self.poultry_g.is_none() {
            missing.push("poultry_g");
        }
        if self.fast_food_g.is_none() {
            missing.push("fast_food_g");
        }
        if self.nuts_g.is_none() {
            missing.push("nuts_g");
        }
        if self.omega3_g.is_none() {
            missing.push("omega3_g");
        }
        if self.vitamin_a_mcg.is_none() {
            missing.push("vitamin_a_mcg");
        }
        if self.vitamin_e_mg.is_none() {
            missing.push("vitamin_e_mg");
        }
        if self.zinc_mg.is_none() {
            missing.push("zinc_mg");
        }
        if self.selenium_mcg.is_none() {
            missing.push("selenium_mcg");
        }
        if self.magnesium_mg.is_none() {
            missing.push("magnesium_mg");
        }
        if self.trans_fat_g.is_none() {
            missing.push("trans_fat_g");
        }
        if self.alcohol_g.is_none() {
            missing.push("alcohol_g");
        }
        missing
    }

    pub fn all_field_names() -> &'static [&'static str] {
        ALL_FIELD_NAMES
    }
}
