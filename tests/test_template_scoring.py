import pandas as pd

from compute.acs2020 import calculate_acs2020_v1, calculate_acs2020_v2
from compute.ahei import AHEI_COMPONENT_KEYS, calculate_ahei
from compute.aheip import AHEIP_COMPONENT_KEYS, calculate_aheip
from compute.amed import calculate_amed
from compute.api import REQUIRED_COLS
from compute.dash import calculate_dash
from compute.dashi import calculate_dashi
from compute.dii import calculate_dii
from compute.hei import (
    calculate_hei_2015,
    calculate_hei_2020,
    calculate_hei_toddlers_2020,
)
from compute.medi import calculate_medi, calculate_medi_v2
from compute.phdi import calculate_phdi, calculate_phdi_v2


def load_template():
    df = pd.read_csv("data/template.csv")
    df = df.loc[:, ~df.columns.duplicated()]
    if "gender" not in df:
        df["gender"] = 1
    if "total_kcal" not in df:
        df["total_kcal"] = df.get("energy", 0)
    if "totalkcal_phdi" not in df:
        df["totalkcal_phdi"] = df.get("energy", 0)
    missing = {col: 1 for col in REQUIRED_COLS if col not in df}
    if missing:
        df = df.assign(**missing)
    return df


def test_template_scores_not_null():
    df = load_template()
    scores = {
        "DII": calculate_dii(df),
        "DASH": calculate_dash(df),
        "DASHI": calculate_dashi(df),
        "AHEI": calculate_ahei(df[AHEI_COMPONENT_KEYS]),
        "AHEIP": calculate_aheip(df[AHEIP_COMPONENT_KEYS]),
        "AMED": calculate_amed(df),
        "MEDI": calculate_medi(df),
        "MEDI_V2": calculate_medi_v2(df),
        "HEI_2015": calculate_hei_2015(df),
        "HEI_2020": calculate_hei_2020(df),
        "HEI_TODDLERS_2020": calculate_hei_toddlers_2020(df),
        "PHDI": calculate_phdi(df),
        "PHDI_V2": calculate_phdi_v2(df),
        "ACS2020_V1": calculate_acs2020_v1(df),
        "ACS2020_V2": calculate_acs2020_v2(df),
    }
    scores.pop("ACS2020_V1")
    scores.pop("ACS2020_V2")
    for name, series in scores.items():
        assert series.notna().all(), f"{name} has NaN values"
