# Scoring Contracts

This document formalizes the expected output for each supported dietary index. For every index the table lists the approximate score range, the set of required input fields, and whether any fallback is allowed when fields are missing.

| Index | Typical Range | Required Inputs | Fallbacks |
|-------|---------------|-----------------|-----------|
| DII | ~-9 to +9 | `DII_PARAMETER_KEYS` | none |
| MIND | 0–15 | `MIND_COMPONENT_KEYS` | none |
| HEI_2015 | 0–100 | `HEI_COMPONENT_KEYS` + `energy_kcal` | none |
| HEI_2020 | 0–100 | `HEI_COMPONENT_KEYS` + `energy_kcal` | none |
| HEI_TODDLERS_2020 | 0–100 | `HEI_COMPONENT_KEYS` + `energy_kcal` | none |
| AHEI | 0–110 | `AHEI_COMPONENT_KEYS` + `gender` | none |
| AHEIP | 0–90 | `AHEIP_COMPONENT_KEYS` | none |
| AMED | 0–9 | `AMED_COMPONENT_KEYS` | none |
| DASH | 8–40 | `DASH_COMPONENT_KEYS` | none |
| DASHI | 0–8 | `DASHI_COMPONENT_KEYS` | none |
| MEDI | 0–9 | `MEDI_COMPONENT_KEYS` | none |
| MEDI_V2 | 0–15 | `MEDI_V2_COMPONENT_KEYS` | none |
| PHDI | 0–15 | `PHDI_COMPONENT_KEYS` + `gender` | none |
| PHDI_V2 | 0–15 | `PHDI_V2_COMPONENT_KEYS` + `gender` | none |
| ACS2020_V1 | 0–15 | `ACS2020_V1_KEYS` + `gender` | none |
| ACS2020_V2 | 0–15 | `ACS2020_V2_KEYS` + `gender` | none |
| ACS2020_V3 | 0–15 | `ACS2020_V3_KEYS` + `gender` | none |

These ranges are approximate based on published methods. The Rust engine will emit structured errors if required fields are missing. No automatic fallback is permitted; all indices must receive complete data.

The canonical list of contract rules, including ranges and required fields, lives in [../schema/contracts.json](../schema/contracts.json). This file is loaded by the Rust engine at runtime so tests and production builds share a single source of truth.
Canonical field names always override any aliases provided during CSV normalization. Aliases exist solely for convenience and never replace their canonical counterparts.
Raw NHANES headers are supported through an additional translation layer so researchers can ingest the original variable names without manual mapping.
ACS-2020 CSV exports from the Adventist Health Study-2 are likewise recognized. Fields ending in `_ACS2020` or matching the official variable list are automatically mapped to the canonical schema so scores compute without manual renaming.
HCSN files follow the same pattern. Column aliases defined in `schema/hcsn_field_aliases.json` are applied on upload so the engine sees the canonical field names.

### HCSN Field Mapping

| HCSN Field | Canonical Field |
|------------|-----------------|
| `leafy_green_veg_servings` | `vegetables_g` |
| `other_veg_servings` | `vegetables_g` |
| `berry_servings` | `berries_g` |
| `nut_servings` | `nuts_g` |
| `bean_servings` | `legumes_g` |
| `whole_grain_g` / `whole_grains_servings` | `whole_grains_g` |
| `fish_servings` | `fish_g` |
| `poultry_servings` | `poultry_g` |
| `wine_servings` | `alcohol_g` |
| `red_meat_servings` | `red_meat_g` |
| `butter_servings` | `butter_g` |
| `cheese_servings` | `cheese_g` |
| `pastry_sweets_servings` | `sugar_g` |
| `fried_food_servings` | `fast_food_g` |
| `olive_oil_daily_use` | `mono_fat_g` |
