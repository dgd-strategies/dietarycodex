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
