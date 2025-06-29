import pandas as pd

from compute.base import validate_dataframe

# Component definitions for MEDI (PREDIMED serving sizes)
# Each component contributes 0 or 1 point based on a cutoff
_MEDI_COMPONENTS = [
    {"key": "olive_oil_serv_medi", "op": ">=", "threshold": 5},
    {"key": "frt_serv_medi", "op": ">=", "threshold": 3},
    {"key": "veg_serv_medi", "op": ">=", "threshold": 2},
    {"key": "legumes_serv_medi", "op": ">=", "threshold": 3 / 7},
    {"key": "nuts_serv_medi", "op": ">=", "threshold": 3 / 7},
    {"key": "fish_seafood_serv_medi", "op": ">=", "threshold": 3 / 7},
    {"key": "alcohol_serv_medi", "op": ">=", "threshold": 1},
    {"key": "ssb_serv_medi", "op": "<", "threshold": 1},
    {"key": "sweets_serv_medi", "op": "<", "threshold": 2 / 7},
    {"key": "discret_fat_serv_medi", "op": "<", "threshold": 1},
    {"key": "redproc_meat_serv_medi", "op": "<", "threshold": 1},
]

# Expose component keys for validation
MEDI_COMPONENT_KEYS = [c["key"] for c in _MEDI_COMPONENTS]

# Component definitions for MEDI_V2 (serving-based 0-5 scoring)
_MEDI_V2_COMPONENTS = [
    {"key": "olive_oil_serv_medi", "type": "healthy", "min": 0.0, "max": 5.0},
    {"key": "frt_serv_medi", "type": "healthy", "min": 0.0, "max": 3.0},
    {"key": "veg_serv_medi", "type": "healthy", "min": 0.0, "max": 2.0},
    {"key": "legumes_serv_medi", "type": "healthy", "min": 0.0, "max": 3 / 7},
    {"key": "nuts_serv_medi", "type": "healthy", "min": 0.0, "max": 3 / 7},
    {
        "key": "fish_seafood_serv_medi",
        "type": "healthy",
        "min": 0.0,
        "max": 3 / 7,
    },
    {"key": "alcohol_serv_medi", "type": "healthy", "min": 0.0, "max": 1.0},
    {"key": "ssb_serv_medi", "type": "unhealthy", "min": 1.0, "max": 0.0},
    {"key": "sweets_serv_medi", "type": "unhealthy", "min": 2 / 7, "max": 0.0},
    {"key": "discret_fat_serv_medi", "type": "unhealthy", "min": 1.0, "max": 0.0},
    {"key": "redproc_meat_serv_medi", "type": "unhealthy", "min": 1.0, "max": 0.0},
]

MEDI_V2_COMPONENT_KEYS = [c["key"] for c in _MEDI_V2_COMPONENTS]


def calculate_medi(df: pd.DataFrame) -> pd.Series:
    """Calculate Mediterranean Diet Index (MEDI) from serving sizes."""
    validate_dataframe(df, MEDI_COMPONENT_KEYS)

    score = pd.Series(0, index=df.index, dtype=float)
    for comp in _MEDI_COMPONENTS:
        col = comp["key"]
        thresh = comp["threshold"]
        if comp["op"] == ">=":
            score += (df[col] >= thresh).astype(int)
        else:
            score += (df[col] < thresh).astype(int)

    return score


def _score_healthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) * 5 / (max_val - min_val)
    score[series >= max_val] = 5.0
    score[series <= min_val] = 0.0
    return score.clip(0.0, 5.0)


def _score_unhealthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) * 5 / (max_val - min_val)
    score[series >= min_val] = 0.0
    score[series <= max_val] = 5.0
    return score.clip(0.0, 5.0)


def calculate_medi_v2(df: pd.DataFrame) -> pd.Series:
    """Calculate Mediterranean Diet Index version 2 (0–5 per component)."""
    validate_dataframe(df, MEDI_V2_COMPONENT_KEYS)

    scores = pd.DataFrame(index=df.index)
    for comp in _MEDI_V2_COMPONENTS:
        if comp["type"] == "healthy":
            scores[comp["key"]] = _score_healthy(
                df[comp["key"]], comp["min"], comp["max"]
            )
        else:
            scores[comp["key"]] = _score_unhealthy(
                df[comp["key"]], comp["min"], comp["max"]
            )

    return scores.sum(axis=1)
