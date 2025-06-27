import pandas as pd

# MIND diet scoring parameters (Morris et al., 2015)
# Components with their weekly serving thresholds for scoring.
# 'healthy' components: higher consumption yields higher score.
# 'unhealthy' components: lower consumption yields higher score.

_MIND_COMPONENTS = [
    # Brain-healthy foods
    {"col": "leafy_green_veg_servings", "type": "healthy", "high": 6, "med": 3},
    {"col": "other_veg_servings", "type": "healthy", "high": 6, "med": 3},
    {"col": "nut_servings", "type": "healthy", "high": 5, "med": 3},
    {"col": "berry_servings", "type": "healthy", "high": 2, "med": 1},
    {"col": "bean_servings", "type": "healthy", "high": 3, "med": 1.5},
    {"col": "whole_grains_servings", "type": "healthy", "high": 3, "med": 1.5},
    {"col": "fish_servings", "type": "healthy", "high": 1, "med": 0.5},
    {"col": "poultry_servings", "type": "healthy", "high": 2, "med": 1},
    {"col": "olive_oil_daily_use", "type": "healthy", "flag": True},
    {"col": "wine_servings", "type": "healthy", "high": 1, "med": 0.5},
    # Unhealthy foods
    {"col": "red_meat_servings", "type": "unhealthy", "high": 0, "med": 1},
    {"col": "butter_servings", "type": "unhealthy", "high": 1, "med": 1.5},
    {"col": "cheese_servings", "type": "unhealthy", "high": 1, "med": 3},
    {"col": "pastry_sweets_servings", "type": "unhealthy", "high": 1, "med": 3},
    {"col": "fried_food_servings", "type": "unhealthy", "high": 1, "med": 3},
]

# Export just the column names for API validation
MIND_COMPONENT_KEYS = [c["col"] for c in _MIND_COMPONENTS]


def calculate_mind(df: pd.DataFrame) -> pd.Series:
    """
    Calculate MIND diet score per Morris et al. (2015).

    Scoring:
      - Healthy components: 1 point if weekly servings >= high,
        0.5 point if between med and high, else 0.
      - Unhealthy components: 1 point if weekly servings <= high,
        0.5 if between high and med, else 0.
      - Olive oil: 1 point if daily use flagged True.

    Total score ranges from 0 to 15.

    Args:
        df: DataFrame with serving columns for each component.
    Returns:
        Series of MIND scores per row.
    """
    scores = pd.Series(0.0, index=df.index)

    for comp in _MIND_COMPONENTS:
        col = comp["col"]
        if col not in df.columns:
            raise KeyError(f"Missing MIND component column: {col}")

        if comp.get("flag"):
            # Olive oil daily use flag (boolean/int)
            scores += df[col].astype(bool).astype(int)
        else:
            values = df[col]
            if comp["type"] == "healthy":
                s = pd.Series(0.0, index=df.index)
                s[values >= comp["high"]] = 1.0
                s[(values >= comp["med"]) & (values < comp["high"])] = 0.5
                scores += s
            else:
                # Unhealthy: lower consumption better
                s = pd.Series(0.0, index=df.index)
                s[values <= comp["high"]] = 1.0
                s[(values > comp["high"]) & (values <= comp["med"])] = 0.5
                scores += s

    return scores
