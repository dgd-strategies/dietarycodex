"""Utilities for renaming common USDA/FNDDS columns.

These mapping dictionaries translate raw column names often found in
USDA or FNDDS exports to the standardized component names used
throughout the scoring modules.
"""

from __future__ import annotations

import logging
from typing import Mapping

import pandas as pd

# Example mappings for several indices -------------------------------------

USDA_HEI_MAP: dict[str, str] = {
    "F_TOTAL": "total_fruit_cup",
    "F_TOTAL_CUP": "total_fruit_cup",
    "F_CITMLB": "whole_fruit_cup",
    "F_OTHER": "whole_fruit_cup",
    "V_TOTAL": "total_veg_cup",
    "V_TOTAL_CUP": "total_veg_cup",
    "V_LEGUMES": "greens_beans_cup",
    "G_WHOLE": "whole_grains_oz",
    "D_TOTAL": "dairy_cup",
    "PF_TOTAL": "protein_oz",
    "PF_SEAFD_HI": "seafood_plant_oz",
    "PF_SEAFD_LOW": "seafood_plant_oz",
    "PF_NUTSDS": "seafood_plant_oz",
    "PF_SOY": "seafood_plant_oz",
    "PF_LEGUMES": "seafood_plant_oz",
    "KCAL": "energy_kcal",
    "SODIUM": "sodium_mg",
    "ADD_SUGARS": "added_sugars_g",
}

USDA_DASH_MAP: dict[str, str] = {
    "F_TOTAL": "fruits_g",
    "F_TOTAL_G": "fruits_g",
    "V_TOTAL": "vegetables_g",
    "V_TOTAL_G": "vegetables_g",
    "G_WHOLE": "whole_grains_g",
    "D_TOTAL": "low_fat_dairy_g",
    "PF_LEGUMES": "nuts_legumes_g",
    "SODIUM": "sodium_mg",
    "PROC_MEAT": "red_processed_meats_g",
    "SLD_BEV": "sweetened_beverages_g",
}

USDA_DII_MAP: dict[str, str] = {
    "ENERGY": "Energy",
    "ENERGY_KCAL": "Energy",
    "PROTEIN": "Protein",
    "TOTALFAT": "Total fat",
    "CARBS": "Carbohydrate",
    "CARBOHYDRATE": "Carbohydrate",
    "FIBER": "Fiber",
    "FOLATE": "Folic acid",
    "VITC": "Vitamin C",
    "VITD": "Vitamin D",
    "VITE": "Vitamin E",
}

# ---------------------------------------------------------------------------


def apply_mapping(df: pd.DataFrame, mapping: Mapping[str, str]) -> pd.DataFrame:
    """Rename DataFrame columns using a mapping and log unmatched fields."""

    rename_map = {src: dst for src, dst in mapping.items() if src in df.columns}
    missing = [src for src in mapping if src not in df.columns]
    unmapped = [
        col for col in df.columns if col not in rename_map and col not in mapping
    ]

    if missing:
        logging.info("Unmapped source columns skipped: %s", sorted(missing))
    if unmapped:
        logging.info("Columns left unmapped: %s", sorted(unmapped))

    return df.rename(columns=rename_map)
