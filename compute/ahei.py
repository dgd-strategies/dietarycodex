import numpy as np
import pandas as pd

from compute.base import validate_dataframe

# Component scoring thresholds
# Scoring range 0-10 for each component
_AHEI_COMPONENTS = [
    {"key": "veg_serv", "type": "positive", "min": 0.0, "max": 5.0},
    {"key": "fruit_serv", "type": "positive", "min": 0.0, "max": 4.0},
    {"key": "whole_grain_g", "type": "positive", "min": 0.0, "max_female": 75.0, "max_male": 90.0},
    {"key": "nuts_legumes_serv", "type": "positive", "min": 0.0, "max": 1.0},
    {"key": "n3_fat_mg", "type": "positive", "min": 0.0, "max": 250.0},
    {"key": "pufa_pct_energy", "type": "positive", "min": 2.0, "max": 10.0},
    {"key": "ssb_fruit_juice_serv", "type": "negative", "min": 1.0, "max": 0.0},
    {"key": "red_processed_meat_serv", "type": "negative", "min": 1.5, "max": 0.0},
    {"key": "trans_fat_pct", "type": "negative", "min": 4.0, "max": 0.5},
    {"key": "alcohol_serv", "type": "alcohol"},
]

# Expose component keys for validation
AHEI_COMPONENT_KEYS = [c["key"] for c in _AHEI_COMPONENTS] + ["sodium_mg", "total_kcal", "gender"]


def _score_linear(val: pd.Series, min_val: float, max_val: float, positive: bool = True) -> pd.Series:
    """Linear scoring between min and max."""
    if positive:
        score = (val - min_val) / (max_val - min_val) * 10
    else:
        score = (max_val - val) / (max_val - min_val) * 10
    return score.clip(0, 10)


def calculate_ahei(df: pd.DataFrame) -> pd.Series:
    """Calculate Alternative Healthy Eating Index (AHEI)."""
    validate_dataframe(df, AHEI_COMPONENT_KEYS)

    scores = pd.DataFrame(index=df.index)

    for comp in _AHEI_COMPONENTS:
        key = comp["key"]
        if comp["type"] == "positive":
            min_val = comp["min"]
            max_val = comp.get("max")
            if key == "whole_grain_g":
                # Gender specific max
                max_vals = np.where(df["gender"] == 2, comp["max_female"], comp["max_male"])
                score = (df[key] - min_val) / (max_vals - min_val) * 10
                scores[key] = score.clip(0, 10)
            else:
                scores[key] = _score_linear(df[key], min_val, max_val, positive=True)
        elif comp["type"] == "negative":
            scores[key] = _score_linear(df[key], comp["min"], comp["max"], positive=False)
        elif comp["type"] == "alcohol":
            # Gender specific optimal range
            score = pd.Series(0.0, index=df.index)
            female = df["gender"] == 2
            male = df["gender"] == 1
            score[female & (df[key] <= 1.5) & (df[key] >= 0.5)] = 10
            score[male & (df[key] <= 2.0) & (df[key] >= 0.5)] = 10
            score[female & (df[key] > 1.5) & (df[key] < 2.5)] = (2.5 - df[key]) / (2.5 - 1.5) * 10
            score[male & (df[key] > 2.0) & (df[key] < 3.5)] = (3.5 - df[key]) / (3.5 - 2.0) * 10
            score[(df[key] < 0.5) & (df[key] > 0)] = df[key] / 0.5 * 10
            score[df[key] == 0] = 2.5
            scores[key] = score.clip(0, 10)

    # Sodium requires cohort-specific deciles per 2000 kcal
    sodium_adj = df["sodium_mg"] / (df["total_kcal"] / 2000)
    deciles = np.quantile(sodium_adj.dropna(), np.linspace(0, 1, 11))
    sodium_score = pd.Series(0.0, index=df.index)
    for i in range(10, 0, -1):
        sodium_score[sodium_adj <= deciles[i]] = 10 - i
    scores["sodium"] = sodium_score

    total = scores.sum(axis=1)
    return total
