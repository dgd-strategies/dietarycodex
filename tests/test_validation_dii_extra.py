import pandas as pd
import pytest

from compute.dii import calculate_dii, get_dii_parameters
from compute.mapping import normalize_dii_headers


def test_dii_matches_publication_validation():
    df = pd.read_csv("data/DII_validation_result.csv")
    df = normalize_dii_headers(df)
    cols = [p["name"] for p in get_dii_parameters()]
    result = calculate_dii(df[cols])
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == df["DII_ALL"].tolist()
