import pandas as pd
import pytest

from compute.unit_conversion import (
    convert_to_canonical_units,
    infer_units,
    rename_for_scoring,
)


def test_infer_and_convert_units():
    df = pd.DataFrame(
        {
            "energy_kj": [418.4],  # 100 kcal
            "vitamin_c_mcg": [1000000],  # 1000 mg
            "protein_g": [10],
        }
    )
    units = infer_units(df)
    assert units["energy"] == "kj"
    assert units["vitamin_c"] == "mcg"
    # After inference the DataFrame should use base names
    assert set(df.columns) == {"energy", "vitamin_c", "protein"}
    df = convert_to_canonical_units(df, units)
    df = rename_for_scoring(df)
    assert df["energy_kcal"].iloc[0] == pytest.approx(100)
    assert df["vitamin_c_mg"].iloc[0] == pytest.approx(1000)
    assert "protein_g" in df.columns


def test_default_unit_suffix_added():
    df = pd.DataFrame({"energy": [50], "protein": [5]})
    units = infer_units(df)
    assert units["energy"] == "kcal"
    df = rename_for_scoring(df)
    assert set(df.columns) == {"energy_kcal", "protein_g"}
