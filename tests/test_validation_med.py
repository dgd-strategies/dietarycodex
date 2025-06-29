import pandas as pd
import pytest

from compute.amed import calculate_amed


def test_med_matches_reference():
    df = pd.read_csv("data/MED_VALIDATION.csv")
    mapping = {
        "fruit": "FRT_FRTJ_SERV_MED",
        "vegetable": "VEG_SERV_MED",
        "whole_grain": "WGRAIN_SERV_MED",
        "legume": "LEGUMES_SERV_MED",
        "nut": "NUTS_SERV_MED",
        "fish": "FISH_SERV_MED",
        "red_processed_meat": "REDPROC_MEAT_SERV_MED",
        "monofat_satfat": "MONSATFAT_SERV_MED",
        "alcohol": "ALCOHOL_SERV_MED",
    }
    df = df.rename(columns=mapping)
    result = calculate_amed(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["EXP_MED_ALL"].tolist()
    )
