import pandas as pd

from compute.base import validate_dataframe

_DASHI_COMPONENTS = [
    {"key": "TOTAL_FAT_DASHI", "type": "unhealthy", "min": 37.0, "max": 27.0},
    {"key": "SAT_FAT_DASHI", "type": "unhealthy", "min": 16.0, "max": 6.0},
    {"key": "PROTEIN_DASHI", "type": "healthy", "min": 15.0, "max": 18.0},
    {"key": "CHOLESTEROL_DASHI", "type": "unhealthy", "min": 285.7, "max": 142.8},
    {"key": "FIBER_DASHI", "type": "healthy", "min": 8.6, "max": 29.5},
    {"key": "POTASSIUM_DASHI", "type": "healthy", "min": 1619.0, "max": 4476.0},
    {"key": "MAGNESIUM_DASHI", "type": "healthy", "min": 157.0, "max": 476.0},
    {"key": "CALCIUM_DASHI", "type": "healthy", "min": 429.0, "max": 1181.0},
    {"key": "SODIUM_DASHI", "type": "unhealthy", "min": 2857.0, "max": 2286.0},
]

DASHI_COMPONENT_KEYS = [c["key"] for c in _DASHI_COMPONENTS]


def _score_healthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) / (max_val - min_val)
    score[series >= max_val] = 1.0
    score[series <= min_val] = 0.0
    return score.clip(0.0, 1.0)


def _score_unhealthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) / (max_val - min_val)
    score[series >= min_val] = 0.0
    score[series <= max_val] = 1.0
    return score.clip(0.0, 1.0)


def calculate_dashi(df: pd.DataFrame) -> pd.Series:
    """Calculate the DASH Index using serving-size cut points from the DASH trial."""
    validate_dataframe(df, DASHI_COMPONENT_KEYS)

    scores = pd.DataFrame(index=df.index)
    for comp in _DASHI_COMPONENTS:
        col = comp["key"]
        if comp["type"] == "healthy":
            scores[col] = _score_healthy(df[col], comp["min"], comp["max"])
        else:
            scores[col] = _score_unhealthy(df[col], comp["min"], comp["max"])

    return scores.sum(axis=1)
