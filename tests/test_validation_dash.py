import pandas as pd
import pytest

from compute.dash import calculate_dash


def test_dash_matches_reference():
    df = pd.read_csv("data/DASH_VALIDATION.csv")
    mapping = {
        "whole_fruit": "fruits",
        "vegetable": "vegetables",
        "nut_legume": "nuts_legumes",
        "whole_grain": "whole_grains",
        "low_fat_dairy": "low_fat_dairy",
        "sodium": "sodium",
        "red_processed_meat": "red_processed_meats",
        "ssb": "sweetened_beverages",
    }
    df = df.rename(columns=mapping)
    result = calculate_dash(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["EXP_DASH_ALL"].tolist()
    )
