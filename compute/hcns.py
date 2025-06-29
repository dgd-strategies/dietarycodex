import pandas as pd

from compute.base import validate_dataframe
from compute.mind import MIND_COMPONENT_KEYS, calculate_mind

# Mapping of HCNS 2013 raw frequency columns to MIND diet component names
HCNS_MIND_MAP = {
    "leafy_green_veg_servings": ["C3A_FF_13"],
    "other_veg_servings": ["C3B_FF_13"],
    "berry_servings": ["C4A_FF_13", "C4B_FF_13"],
    "nut_servings": ["C5A_FF_13", "C5B_FF_13", "C5C_FF_13", "C5D_FF_13"],
    "whole_grains_servings": ["C6A_FF_13", "C6B_FF_13", "C6C_FF_13", "C6D_FF_13"],
    "fish_servings": ["C7A_FF_13", "C7B_FF_13"],
    "bean_servings": ["C8A_FF_13", "C8B_FF_13", "C8C_FF_13"],
    "poultry_servings": ["C7E_FF_13", "C7F_FF_13"],
    "olive_oil_daily_use": ["C9A_FF_13"],
    "wine_servings": ["C9B_FF_13"],
    "red_meat_servings": ["C9C_FF_13", "C9D_FF_13", "C9E_FF_13"],
    "butter_servings": ["C9F_FF_13"],
    "cheese_servings": ["C9G_FF_13", "C9H_FF_13"],
    "pastry_sweets_servings": ["C9I_FF_13", "C9J_FF_13", "C9K_FF_13"],
    "fried_food_servings": ["C9L_FF_13", "C9M_FF_13"],
}


def aggregate_hcns_to_mind(
    df: pd.DataFrame, mapping: dict[str, list[str]] | None = None
) -> pd.DataFrame:
    """Aggregate HCNS 2013 columns into MIND component servings."""
    if mapping is None:
        mapping = HCNS_MIND_MAP
    required = [c for cols in mapping.values() for c in cols]
    validate_dataframe(df, required)

    result = pd.DataFrame(index=df.index)
    for target, cols in mapping.items():
        if target == "olive_oil_daily_use":
            result[target] = df[cols[0]].astype(bool).astype(int)
        else:
            result[target] = df[cols].sum(axis=1)
    return result[MIND_COMPONENT_KEYS]


def calculate_mind_from_hcns(df: pd.DataFrame) -> pd.Series:
    """Calculate MIND score directly from HCNS 2013 raw columns."""
    servings = aggregate_hcns_to_mind(df)
    return calculate_mind(servings)
