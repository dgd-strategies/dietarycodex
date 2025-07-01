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
    "KCAL": "energy",
    "SODIUM": "sodium_mg",
    "ADD_SUGARS": "added_sugars_g",
}

USDA_DASH_MAP: dict[str, str] = {
    "F_TOTAL": "fruits",
    "F_TOTAL_G": "fruits",
    "V_TOTAL": "vegetables",
    "V_TOTAL_G": "vegetables",
    "G_WHOLE": "whole_grains",
    "D_TOTAL": "low_fat_dairy",
    "PF_LEGUMES": "nuts_legumes",
    "SODIUM": "sodium",
    "PROC_MEAT": "red_processed_meats",
    "SLD_BEV": "sweetened_beverages",
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

# Column case fixes for DII validation datasets
DII_CASE_MAP: dict[str, str] = {
    "vitamin B12": "Vitamin B12",
    "vitamin B6": "Vitamin B6",
    "Thyme_oregano": "Thyme/oregano",
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


def normalize_dii_headers(df: pd.DataFrame) -> pd.DataFrame:
    """Normalize common DII column casing variations."""
    return apply_mapping(df, DII_CASE_MAP)
