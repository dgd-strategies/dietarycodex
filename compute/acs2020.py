import pandas as pd

from compute.base import validate_dataframe

# Components for ACS2020 version 1 and 2
ACS2020_V1_KEYS = [
    "gender",
    "VEG_SERV_ACS2020",
    "VEG_ITEMS_SERV_ACS2020",
    "FRT_SERV_ACS2020",
    "FRT_ITEMS_SERV_ACS2020",
    "WGRAIN_SERV_ACS2020",
    "REDPROC_MEAT_SERV_ACS2020",
    "HPFRG_RATIO_SERV_ACS2020",
    "SSB_FRTJ_SERV_ACS2020",
]

ACS2020_V2_KEYS = ACS2020_V1_KEYS[:-2] + [
    "HPFRG_SERV_ACS2020",
    "SSB_FRTJ_SERV_ACS2020",
    "TOTALKCAL_ACS2020",
]

# Placeholder components for a future ACS2020 version 3 implementation.
ACS2020_V3_KEYS = ACS2020_V2_KEYS


def _quartile_ranks(series: pd.Series, gender: pd.Series) -> pd.Series:
    def rank_group(s: pd.Series) -> pd.Series:
        try:
            return pd.qcut(s, 4, labels=False, duplicates="drop")
        except Exception:
            return (s.rank(pct=True) * 4).clip(0, 3).fillna(1).astype(int)

    return series.groupby(gender).transform(rank_group)


def _map_scores(ranks: pd.Series, mapping: list[float]) -> pd.Series:
    return ranks.map(lambda r: mapping[int(r)] if pd.notna(r) else pd.NA)


def calculate_acs2020_v1(df: pd.DataFrame) -> pd.Series:
    """Calculate American Cancer Society 2020 diet score (version 1)."""
    validate_dataframe(df, ACS2020_V1_KEYS)

    gender = df["gender"]

    veg = _map_scores(
        _quartile_ranks(df["VEG_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    veg_items = _map_scores(
        _quartile_ranks(df["VEG_ITEMS_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    frt = _map_scores(
        _quartile_ranks(df["FRT_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    frt_items = _map_scores(
        _quartile_ranks(df["FRT_ITEMS_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    wgrain = _map_scores(
        _quartile_ranks(df["WGRAIN_SERV_ACS2020"], gender), [0, 1, 2, 3]
    )
    red_meat = _map_scores(
        _quartile_ranks(df["REDPROC_MEAT_SERV_ACS2020"], gender), [3, 2, 1, 0]
    )
    hpfrg = _map_scores(
        _quartile_ranks(df["HPFRG_RATIO_SERV_ACS2020"], gender), [1.5, 1, 0.5, 0]
    )

    ssb = pd.Series(0.0, index=df.index)
    ssb[df["SSB_FRTJ_SERV_ACS2020"] >= 1] = 0
    ssb[(df["SSB_FRTJ_SERV_ACS2020"] < 1) & (df["SSB_FRTJ_SERV_ACS2020"] >= 0.428)] = (
        0.5
    )
    ssb[(df["SSB_FRTJ_SERV_ACS2020"] < 0.428) & (df["SSB_FRTJ_SERV_ACS2020"] > 0)] = 1
    ssb[df["SSB_FRTJ_SERV_ACS2020"] <= 0] = 1.5

    total = veg + veg_items + frt + frt_items + wgrain + red_meat + hpfrg + ssb
    return total


def calculate_acs2020_v2(df: pd.DataFrame) -> pd.Series:
    """Calculate American Cancer Society 2020 diet score (version 2)."""
    validate_dataframe(df, ACS2020_V2_KEYS)

    gender = df["gender"]

    veg = _map_scores(
        _quartile_ranks(df["VEG_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    veg_items = _map_scores(
        _quartile_ranks(df["VEG_ITEMS_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    frt = _map_scores(
        _quartile_ranks(df["FRT_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    frt_items = _map_scores(
        _quartile_ranks(df["FRT_ITEMS_SERV_ACS2020"], gender), [0, 0.25, 0.5, 0.75]
    )
    wgrain = _map_scores(
        _quartile_ranks(df["WGRAIN_SERV_ACS2020"], gender), [0, 1, 2, 3]
    )
    red_meat = _map_scores(
        _quartile_ranks(df["REDPROC_MEAT_SERV_ACS2020"], gender), [3, 2, 1, 0]
    )

    hpfrg_serv = df["HPFRG_SERV_ACS2020"] / (df["TOTALKCAL_ACS2020"] / 1000)
    hpfrg = _map_scores(_quartile_ranks(hpfrg_serv, gender), [1.5, 1, 0.5, 0])

    ssb = pd.Series(0.0, index=df.index)
    ssb[df["SSB_FRTJ_SERV_ACS2020"] >= 1] = 0
    ssb[(df["SSB_FRTJ_SERV_ACS2020"] < 1) & (df["SSB_FRTJ_SERV_ACS2020"] >= 0.428)] = (
        0.5
    )
    ssb[(df["SSB_FRTJ_SERV_ACS2020"] < 0.428) & (df["SSB_FRTJ_SERV_ACS2020"] > 0)] = 1
    ssb[df["SSB_FRTJ_SERV_ACS2020"] <= 0] = 1.5

    total = veg + veg_items + frt + frt_items + wgrain + red_meat + hpfrg + ssb
    return total


def calculate_acs2020_v3(df: pd.DataFrame) -> pd.Series:
    """Placeholder for ACS2020 version 3.

    This stub reuses the version 2 algorithm until official cut points are
    finalized.
    """
    validate_dataframe(df, ACS2020_V3_KEYS)
    return calculate_acs2020_v2(df)
