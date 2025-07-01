import json

import pandas as pd
import pytest

from compute.acs2020 import calculate_acs2020_v2
from compute.mapping import apply_mapping

with open("schema/acs2020_field_aliases.json") as f:
    ACS_MAP = json.load(f)


def make_df():
    return pd.read_csv("data/ACS2020_V2_validation.csv").drop(
        columns=[
            "EXP_ACS_ALL",
            "EXP_ACS_VEGETABLE",
            "EXP_ACS_VEGETABLE_UNIQUE",
            "EXP_ACS_FRUIT",
            "EXP_ACS_FRUIT_UNIQUE",
            "EXP_ACS_WHOLE_GRAIN",
            "EXP_ACS_RED_MEAT",
            "EXP_ACS_PROCESSED_FOOD",
            "EXP_ACS_SSB",
        ]
    )


def test_alias_normalization_scores():
    df = make_df()
    df = apply_mapping(df, ACS_MAP)
    v2 = calculate_acs2020_v2(df)
    assert v2.notna().all()


def test_missing_alias_triggers_error():
    df = make_df().drop(columns=["fruit"])
    df = apply_mapping(df, ACS_MAP)
    with pytest.raises(Exception):
        calculate_acs2020_v2(df)
