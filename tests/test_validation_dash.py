import pandas as pd
import pytest

from compute.dash import calculate_dash


def test_dash_matches_reference():
    df = pd.read_csv("data/DASH_VALIDATION.csv")
    mapping = {
        "whole_fruit": "fruits_g",
        "vegetable": "vegetables_g",
        "nut_legume": "nuts_legumes_g",
        "whole_grain": "whole_grains_g",
        "low_fat_dairy": "low_fat_dairy_g",
        "sodium": "sodium_mg",
        "red_processed_meat": "red_processed_meats_g",
        "ssb": "sweetened_beverages_g",
    }
    df = df.rename(columns=mapping)
    result = calculate_dash(df)
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["EXP_DASH_ALL"].tolist()
