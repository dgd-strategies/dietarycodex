import pandas as pd

from compute.base import validate_dataframe

_AMED_COMPONENTS = [
    {"key": "FRT_FRTJ_SERV_MED", "type": "healthy"},
    {"key": "VEG_SERV_MED", "type": "healthy"},
    {"key": "WGRAIN_SERV_MED", "type": "healthy"},
    {"key": "LEGUMES_SERV_MED", "type": "healthy"},
    {"key": "NUTS_SERV_MED", "type": "healthy"},
    {"key": "FISH_SERV_MED", "type": "healthy"},
    {"key": "REDPROC_MEAT_SERV_MED", "type": "unhealthy"},
    {"key": "MONSATFAT_SERV_MED", "type": "healthy"},
    {"key": "ALCOHOL_SERV_MED", "type": "alcohol"},
]

AMED_COMPONENT_KEYS = [c["key"] for c in _AMED_COMPONENTS]


def calculate_amed(df: pd.DataFrame) -> pd.Series:
    """Calculate the Alternate Mediterranean Diet Score (aMED)."""
    validate_dataframe(df, AMED_COMPONENT_KEYS)

    scores = pd.DataFrame(index=df.index)
    for comp in _AMED_COMPONENTS:
        col = comp["key"]
        if comp["type"] == "healthy":
            median = df[col].median(skipna=True)
            scores[col] = ((df[col] >= median) & (df[col] > 0)).astype(int)
        elif comp["type"] == "unhealthy":
            median = df[col].median(skipna=True)
            scores[col] = ((df[col] < median) | (df[col] == 0)).astype(int)
        else:  # alcohol
            scores[col] = ((df[col] >= 10) & (df[col] <= 25)).astype(int)

    return scores.sum(axis=1)
