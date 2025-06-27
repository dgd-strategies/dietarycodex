import pandas as pd

from compute.base import validate_dataframe

# HEI-2015 component scoring standards per 1000 kcal or ratio-based
_HEI_COMPONENTS = [
    # Adequacy components: density per 1000 kcal
    {"key": "total_fruit_cup", "type": "adequacy", "min": 0.0, "max": 0.8, "points": 5},
    {"key": "whole_fruit_cup", "type": "adequacy", "min": 0.0, "max": 0.4, "points": 5},
    {"key": "total_veg_cup", "type": "adequacy", "min": 0.0, "max": 1.1, "points": 5},
    {
        "key": "greens_beans_cup",
        "type": "adequacy",
        "min": 0.0,
        "max": 0.2,
        "points": 5,
    },
    {
        "key": "whole_grains_oz",
        "type": "adequacy",
        "min": 0.0,
        "max": 1.5,
        "points": 10,
    },
    {"key": "dairy_cup", "type": "adequacy", "min": 0.0, "max": 1.3, "points": 10},
    {"key": "protein_oz", "type": "adequacy", "min": 0.0, "max": 2.5, "points": 5},
    {
        "key": "seafood_plant_oz",
        "type": "adequacy",
        "min": 0.0,
        "max": 0.8,
        "points": 5,
    },
    # Ratio component: (MUFA + PUFA) / SFA
    {
        "key": ["monounsaturated_fat_g", "polyunsaturated_fat_g", "saturated_fat_g"],
        "type": "ratio",
        "min": 1.2,
        "max": 2.5,
        "points": 10,
    },
    # Moderation components: lower density is better
    {
        "key": "refined_grains_oz",
        "type": "moderation",
        "min": 1.8,
        "max": 4.3,
        "points": 10,
    },
    {"key": "sodium_mg", "type": "moderation", "min": 1100, "max": 2300, "points": 10},
    # Percent of energy components
    {
        "key": "added_sugars_g",
        "type": "percent_kcal",
        "min": 0.0,
        "max": 10.0,
        "points": 10,
    },
    {
        "key": "saturated_fat_g",
        "type": "percent_kcal",
        "min": 0.0,
        "max": 8.0,
        "points": 10,
    },
]

# Expose component keys for API validation
HEI_COMPONENT_KEYS = []
for comp in _HEI_COMPONENTS:
    if isinstance(comp["key"], list):
        HEI_COMPONENT_KEYS.extend(comp["key"])
    else:
        HEI_COMPONENT_KEYS.append(comp["key"])
# Ensure energy is included
HEI_COMPONENT_KEYS.append("energy_kcal")


def calculate_hei_2015(df: pd.DataFrame) -> pd.Series:
    """
    Calculate Healthy Eating Index-2015 (HEI-2015) total score.

    Args:
        df: pandas DataFrame containing HEI component columns and 'energy_kcal'.
    Returns:
        pandas Series of HEI-2015 scores (0-100) per row.
    """
    # Validate inputs
    validate_dataframe(df, HEI_COMPONENT_KEYS)

    energy = df["energy_kcal"]
    scores = pd.DataFrame(index=df.index)

    for comp in _HEI_COMPONENTS:
        pts = comp["points"]
        ctype = comp["type"]
        key = comp["key"]

        if ctype == "adequacy":
            val = df[key] / energy * 1000
            sc = ((val - comp["min"]) / (comp["max"] - comp["min"]) * pts).clip(0, pts)

        elif ctype == "ratio":
            mu, pu, sa = df[key[0]], df[key[1]], df[key[2]]
            ratio = (mu + pu) / sa.replace(0, pd.NA)
            sc = ((ratio - comp["min"]) / (comp["max"] - comp["min"]) * pts).clip(
                0, pts
            )

        elif ctype == "moderation":
            val = df[key] / energy * 1000
            sc = ((comp["max"] - val) / (comp["max"] - comp["min"]) * pts).clip(0, pts)

        elif ctype == "percent_kcal":
            pct = df[key] * 4 / energy * 100
            sc = ((comp["max"] - pct) / (comp["max"] - comp["min"]) * pts).clip(0, pts)

        else:
            raise ValueError(f"Unknown HEI component type: {ctype}")

        # Assign score column
        col_name = key[0] if isinstance(key, list) else key
        scores[col_name] = sc

    # Sum all component scores into total
    total_score = scores.sum(axis=1)
    return total_score
