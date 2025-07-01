import pandas as pd
import pytest

from compute.ahei import calculate_ahei
from compute.aheip import calculate_aheip


def test_ahei_matches_reference():
    df = pd.read_csv("data/AHEI_VALIDATION.csv")
    mapping = {
        "vegetable": "veg_serv",
        "fruit": "fruit_serv",
        "whole_grain": "whole_grain",
        "nut_legume": "nuts_legumes_serv",
        "n3_fat": "n3_fat",
        "pufa": "pufa_pct_energy",
        "ssb_fruit_juice": "ssb_fruit_juice_serv",
        "red_processed_meat": "red_processed_meat_serv",
        "trans_fat": "trans_fat_pct",
        "sodium": "sodium",
        "alcohol": "alcohol_serv",
        "kcal": "total_kcal",
    }
    df = df.rename(columns=mapping)
    result = calculate_ahei(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["EXP_AHEI_ALL"].tolist()
    )


def test_aheip_matches_reference():
    df = pd.read_csv("data/AHEIP_VALIDATION.csv")
    mapping = {
        "vegetable": "VEG_SERV_AHEIP",
        "whole_fruit": "FRT_SERV_AHEIP",
        "white_meat_red_meat": "WHITERED_RT_SERV_AHEIP",
        "fiber": "FIBER_SERV_AHEIP",
        "trans_fat": "TRANS_SERV_AHEIP",
        "poly_fat_sat_fat": "POLYSAT_RT_SERV_AHEIP",
        "calcium": "CALCIUM_SERV_AHEIP",
        "folate": "FOLATE_SERV_AHEIP",
        "iron": "IRON_SERV_AHEIP",
    }
    df = df.rename(columns=mapping)
    result = calculate_aheip(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["EXP_AHEIP_ALL"].tolist()
    )
