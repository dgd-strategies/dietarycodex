import numpy as np
import pandas as pd

from compute.base import validate_dataframe

# Component thresholds for Planetary Health Diet Index (PHDI)
# Values in grams/day unless noted otherwise
_PHDI_COMPONENTS = [
    {"key": "wgrain_serv_phdi", "type": "wgrain"},
    {"key": "starchy_veg_serv_phdi", "type": "unhealthy", "min": 200.0, "max": 50.0},
    {"key": "veg_serv_phdi", "type": "healthy", "min": 0.0, "max": 300.0},
    {"key": "frt_serv_phdi", "type": "healthy", "min": 0.0, "max": 200.0},
    {"key": "dairy_serv_phdi", "type": "unhealthy", "min": 1000.0, "max": 250.0},
    {"key": "redproc_meat_serv_phdi", "type": "unhealthy", "min": 100.0, "max": 14.0},
    {"key": "poultry_serv_phdi", "type": "unhealthy", "min": 100.0, "max": 29.0},
    {"key": "egg_serv_phdi", "type": "unhealthy", "min": 120.0, "max": 13.0},
    {"key": "fish_serv_phdi", "type": "healthy", "min": 0.0, "max": 28.0},
    {"key": "nuts_serv_phdi", "type": "healthy", "min": 0.0, "max": 50.0},
    {"key": "legumes_serv_phdi", "type": "healthy5", "min": 0.0, "max": 100.0},
    {"key": "soy_serv_phdi", "type": "healthy5", "min": 0.0, "max": 50.0},
    {"key": "added_fat_unsat_serv_phdi", "type": "healthy", "min": 3.5, "max": 21.0},
    {
        "key": "added_fat_sat_trans_serv_phdi",
        "type": "unhealthy",
        "min": 10.0,
        "max": 0.0,
    },
    {"key": "added_sugar_serv_phdi", "type": "unhealthy", "min": 25.0, "max": 5.0},
]

# Expose for validation
PHDI_COMPONENT_KEYS = [c["key"] for c in _PHDI_COMPONENTS] + ["gender"]

# Input columns for the gram-based PHDI_V2 version include total energy
PHDI_V2_COMPONENT_KEYS = PHDI_COMPONENT_KEYS + ["totalkcal_phdi"]


def _score_healthy(
    series: pd.Series, min_val: float, max_val: float, max_score: float
) -> pd.Series:
    score = (series - min_val) * max_score / (max_val - min_val)
    score[series >= max_val] = max_score
    score[series <= min_val] = 0.0
    return score.clip(0.0, max_score)


def _score_unhealthy(
    series: pd.Series, min_val: float, max_val: float, max_score: float
) -> pd.Series:
    score = (series - min_val) * max_score / (max_val - min_val)
    score[series >= min_val] = 0.0
    score[series <= max_val] = max_score
    return score.clip(0.0, max_score)


def calculate_phdi(df: pd.DataFrame) -> pd.Series:
    """Calculate Planetary Health Diet Index (PHDI)."""
    validate_dataframe(df, PHDI_COMPONENT_KEYS)

    gender = df["gender"]
    scores = pd.DataFrame(index=df.index)

    for comp in _PHDI_COMPONENTS:
        key = comp["key"]
        max_score = 10.0
        if comp["type"] == "wgrain":
            max_vals = np.where(gender == 2, 75.0, 90.0)
            score = (df[key] - 0.0) * max_score / (max_vals - 0.0)
            score[df[key] >= max_vals] = max_score
            score[df[key] <= 0.0] = 0.0
            scores[key] = pd.Series(score, index=df.index).clip(0.0, max_score)
        else:
            if comp["type"] == "healthy":
                scores[key] = _score_healthy(
                    df[key], comp["min"], comp["max"], max_score
                )
            elif comp["type"] == "healthy5":
                scores[key] = _score_healthy(df[key], comp["min"], comp["max"], 5.0)
            else:
                scores[key] = _score_unhealthy(
                    df[key], comp["min"], comp["max"], max_score
                )

    total = scores.sum(axis=1)
    return total


def calculate_phdi_v2(df: pd.DataFrame) -> pd.Series:
    """Calculate PHDI using gram-based fat and sugar inputs."""
    validate_dataframe(df, PHDI_V2_COMPONENT_KEYS)

    df = df.copy()
    df["added_fat_unsat_serv_phdi"] = (
        df["added_fat_unsat_serv_phdi"] * 9 / df["totalkcal_phdi"] * 100
    )
    df["added_fat_sat_trans_serv_phdi"] = (
        df["added_fat_sat_trans_serv_phdi"] * 9 / df["totalkcal_phdi"] * 100
    )
    df["added_sugar_serv_phdi"] = (
        df["added_sugar_serv_phdi"] * 4 / df["totalkcal_phdi"] * 100
    )

    return calculate_phdi(df)
