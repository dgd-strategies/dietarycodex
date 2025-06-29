import numpy as np
import pandas as pd
import pytest

from compute.acs2020 import (
    ACS2020_V1_KEYS,
    ACS2020_V2_KEYS,
    calculate_acs2020_v1,
    calculate_acs2020_v2,
)
from compute.ahei import AHEI_COMPONENT_KEYS, calculate_ahei
from compute.aheip import AHEIP_COMPONENT_KEYS, calculate_aheip
from compute.dash import DASH_COMPONENT_KEYS, calculate_dash
from compute.dii import calculate_dii, get_dii_parameters
from compute.hei import (
    HEI_COMPONENT_KEYS,
    calculate_hei_2015,
    calculate_hei_2020,
    calculate_hei_toddlers_2020,
)
from compute.medi import (
    MEDI_COMPONENT_KEYS,
    MEDI_V2_COMPONENT_KEYS,
    calculate_medi,
    calculate_medi_v2,
)
from compute.mind import MIND_COMPONENT_KEYS, calculate_mind
from compute.phdi import PHDI_COMPONENT_KEYS, calculate_phdi


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


def test_hei2020_output_length():
    cols = HEI_COMPONENT_KEYS + ["energy_kcal"]
    df = make_dummy_df(cols, n=5)
    result = calculate_hei_2020(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 5


def test_hei_toddlers_2020_output_length():
    cols = HEI_COMPONENT_KEYS + ["energy_kcal"]
    df = make_dummy_df(cols, n=4)
    result = calculate_hei_toddlers_2020(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 4


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


def test_ahei_output_length():
    cols = AHEI_COMPONENT_KEYS
    df = make_dummy_df(cols, n=6)
    df["gender"] = 1
    result = calculate_ahei(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 6


def test_aheip_output_length():
    cols = AHEIP_COMPONENT_KEYS
    df = make_dummy_df(cols, n=5)
    result = calculate_aheip(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 5


def test_medi_output_length():
    cols = MEDI_COMPONENT_KEYS
    df = make_dummy_df(cols, n=4)
    result = calculate_medi(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 4


def test_medi_v2_output_length():
    cols = MEDI_V2_COMPONENT_KEYS
    df = make_dummy_df(cols, n=4)
    result = calculate_medi_v2(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 4


def test_phdi_output_length():
    cols = PHDI_COMPONENT_KEYS
    df = make_dummy_df(cols, n=3)
    df["gender"] = 1
    result = calculate_phdi(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 3


def test_acs2020_v1_output_length():
    cols = ACS2020_V1_KEYS
    df = make_dummy_df(cols, n=2)
    df["gender"] = 1
    result = calculate_acs2020_v1(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 2


def test_acs2020_v2_output_length():
    cols = ACS2020_V2_KEYS
    df = make_dummy_df(cols, n=2)
    df["gender"] = 1
    df["TOTALKCAL_ACS2020"] = 2000
    result = calculate_acs2020_v2(df)
    assert isinstance(result, pd.Series)
    assert len(result) == 2
