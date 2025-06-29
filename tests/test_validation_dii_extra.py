import pandas as pd
import pytest

from compute.dii import calculate_dii, get_dii_parameters


def test_dii_matches_publication_validation():
    df = pd.read_csv("data/DII_validation_result.csv")
    df = df.rename(
        columns={
            "vitamin B12": "Vitamin B12",
            "vitamin B6": "Vitamin B6",
            "Thyme_oregano": "Thyme/oregano",
        }
    )
    cols = [p["name"] for p in get_dii_parameters()]
    result = calculate_dii(df[cols])
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["DII_ALL"].tolist()
