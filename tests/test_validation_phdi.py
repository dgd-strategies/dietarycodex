import pandas as pd
import pytest

from compute.phdi import calculate_phdi, calculate_phdi_v2


def test_phdi_matches_reference():
    df = pd.read_csv("data/PHDI_VALIDATION.csv")
    result = calculate_phdi(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["exp_phdi_all"].tolist()
    )


def test_phdi_v2_matches_reference():
    df = pd.read_csv("data/PHDI_V2_VALIDATION.csv")
    result = calculate_phdi_v2(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["exp_phdi_all"].tolist()
    )
