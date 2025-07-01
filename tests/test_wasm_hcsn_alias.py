import subprocess
from pathlib import Path


def test_hcsn_aliases(tmp_path):
    script = tmp_path / "run_hcsn.js"
    repo = Path(__file__).resolve().parents[1]
    script.write_text(
        f"""
const fs = require('fs');
const {{ initSync, score_json }} = require('{repo}/assets/wasm/dietarycodex.js');
const b64 = fs.readFileSync('{repo}/assets/wasm/dietarycodex_bg.wasm.b64', 'utf8');
initSync(Buffer.from(b64, 'base64'));
const data = JSON.stringify([{{
  leafy_green_veg_servings: 5,
  butter_servings: 1,
  whole_grain_g: 30,
  energy_kcal: 1,
  fat_g: 1,
  saturated_fat_g: 1,
  carbs_g: 1,
  fiber_g: 1,
  sugar_g: 1,
  protein_g: 1,
  sodium_mg: 1,
  calcium_mg: 1,
  iron_mg: 1,
  vitamin_c_mg: 1,
  total_fruits_g: 1,
  vegetables_g: 1,
  whole_grains_g: 1,
  refined_grains_g: 1,
  legumes_g: 1,
  fish_g: 1,
  red_meat_g: 1,
  mono_fat_g: 1,
  berries_g: 1,
  cheese_g: 1,
  butter_g: 1,
  poultry_g: 1,
  fast_food_g: 1,
  nuts_g: 1,
  omega3_g: 1,
  vitamin_a_mcg: 1,
  vitamin_e_mg: 1,
  zinc_mg: 1,
  selenium_mcg: 1,
  magnesium_mg: 1,
  trans_fat_g: 1,
  alcohol_g: 1
}}]);
const raw = score_json(data);
const scores = Object.fromEntries(raw.rows[0].scores);
console.log(scores.MIND !== undefined);
"""
    )
    output = subprocess.check_output(["node", script], text=True).strip()
    assert output == "true"
