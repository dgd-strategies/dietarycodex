import pandas as pd

from compute.base import validate_dataframe

_AHEIP_COMPONENTS = [
    {"key": "VEG_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 5.0},
    {"key": "FRT_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 4.0},
    {"key": "WHITERED_RT_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 4.0},
    {"key": "FIBER_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 25.0},
    {"key": "TRANS_SERV_AHEIP", "type": "unhealthy", "min": 4.0, "max": 0.5},
    {"key": "POLYSAT_RT_SERV_AHEIP", "type": "healthy", "min": 0.1, "max": 1.0},
    {"key": "CALCIUM_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 1200.0},
    {"key": "FOLATE_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 600.0},
    {"key": "IRON_SERV_AHEIP", "type": "healthy", "min": 0.0, "max": 27.0},
]

AHEIP_COMPONENT_KEYS = [c["key"] for c in _AHEIP_COMPONENTS]


def _score_healthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) * 10 / (max_val - min_val)
    score[series >= max_val] = 10.0
    score[series <= min_val] = 0.0
    return score.clip(0.0, 10.0)


def _score_unhealthy(series: pd.Series, min_val: float, max_val: float) -> pd.Series:
    score = (series - min_val) * 10 / (max_val - min_val)
    score[series >= min_val] = 0.0
    score[series <= max_val] = 10.0
    return score.clip(0.0, 10.0)


def calculate_aheip(df: pd.DataFrame) -> pd.Series:
    """Calculate serving-based Alternative Healthy Eating Index (AHEIP)."""
    validate_dataframe(df, AHEIP_COMPONENT_KEYS)

    scores = pd.DataFrame(index=df.index)
    for comp in _AHEIP_COMPONENTS:
        key = comp["key"]
        if comp["type"] == "healthy":
            scores[key] = _score_healthy(df[key], comp["min"], comp["max"])
        else:
            scores[key] = _score_unhealthy(df[key], comp["min"], comp["max"])

    return scores.sum(axis=1)
