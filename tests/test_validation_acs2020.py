import pandas as pd
import pytest

from compute.acs2020 import calculate_acs2020_v1, calculate_acs2020_v2


def test_acs2020_v1_matches_reference():
    df = pd.read_csv("ACS2020_V1_validation.csv")
    mapping = {
        "vegetable": "VEG_SERV_ACS2020",
        "vegetable_unique": "VEG_ITEMS_SERV_ACS2020",
        "fruit": "FRT_SERV_ACS2020",
        "fruit_unique": "FRT_ITEMS_SERV_ACS2020",
        "whole_grain": "WGRAIN_SERV_ACS2020",
        "red_meat": "REDPROC_MEAT_SERV_ACS2020",
        "process_food": "HPFRG_RATIO_SERV_ACS2020",
        "ssb": "SSB_FRTJ_SERV_ACS2020",
    }
    df = df.rename(columns=mapping)
    result = calculate_acs2020_v1(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["EXP_ACS_ALL"].tolist()
    )


def test_acs2020_v2_matches_reference():
    df = pd.read_csv("ACS2020_V2_validation.csv")
    mapping = {
        "vegetable": "VEG_SERV_ACS2020",
        "vegetable_unique": "VEG_ITEMS_SERV_ACS2020",
        "fruit": "FRT_SERV_ACS2020",
        "fruit_unique": "FRT_ITEMS_SERV_ACS2020",
        "whole_grain": "WGRAIN_SERV_ACS2020",
        "red_meat": "REDPROC_MEAT_SERV_ACS2020",
        "process_food": "HPFRG_SERV_ACS2020",
        "ssb": "SSB_FRTJ_SERV_ACS2020",
        "kcal": "TOTALKCAL_ACS2020",
    }
    df = df.rename(columns=mapping)
    result = calculate_acs2020_v2(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["EXP_ACS_ALL"].tolist()
    )
