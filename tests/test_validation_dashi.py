import pandas as pd
import pytest

from compute.dashi import calculate_dashi


def test_dashi_matches_reference():
    df = pd.read_csv("data/DASHI_VALIDATION.csv")
    mapping = {
        "total_fat": "TOTAL_FAT_DASHI",
        "sat_fat": "SAT_FAT_DASHI",
        "protein": "PROTEIN_DASHI",
        "cholesterol": "CHOLESTEROL_DASHI",
        "fiber": "FIBER_DASHI",
        "potassium": "POTASSIUM_DASHI",
        "magnesium": "MAGNESIUM_DASHI",
        "calcium": "CALCIUM_DASHI",
        "sodium": "SODIUM_DASHI",
    }
    df = df.rename(columns=mapping)
    result = calculate_dashi(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["EXP_DASHI_ALL"].tolist()
    )
