import numpy as np
import pandas as pd
import pytest

from compute.dash import DASH_COMPONENT_KEYS, calculate_dash
from compute.dii import calculate_dii, get_dii_parameters
from compute.hei import HEI_COMPONENT_KEYS, calculate_hei_2015
from compute.mind import MIND_COMPONENT_KEYS, calculate_mind


def make_dummy_df(cols, n=10, fill_value=1.0):
    """
    Create a dummy DataFrame with given columns and constant values.
    """
    return pd.DataFrame({col: np.full(n, fill_value) for col in cols})


def test_dii_output_length_and_zero_mean():
    params = get_dii_parameters()
    keys = [p["name"] for p in params]
    df = make_dummy_df(keys, n=5, fill_value=params[0]["mean"])
    # Overwrite all columns to their respective means for zero-score expectation
    for p in params:
        df[p["name"]] = p["mean"]
    result = calculate_dii(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 5
    # All means => DII ~ 0
    assert pytest.approx(0.0, abs=1e-6) == result.iloc[0]


def test_hei_output_length():
    cols = HEI_COMPONENT_KEYS + ["energy_kcal"]
    df = make_dummy_df(cols, n=8)
    result = calculate_hei_2015(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 8


def test_mind_output_length():
    cols = MIND_COMPONENT_KEYS
    df = make_dummy_df(cols, n=7)
    result = calculate_mind(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 7


def test_dash_output_length():
    cols = DASH_COMPONENT_KEYS
    df = make_dummy_df(cols, n=6)
    result = calculate_dash(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 6
