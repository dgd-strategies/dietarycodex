import pandas as pd
import pytest

from compute.hei import (
    calculate_hei_2015,
    calculate_hei_2020,
    calculate_hei_toddlers_2020,
)


def _prepare_common(df):
    mapping = {
        "kcal": "energy_kcal",
        "total_fruit": "total_fruit_cup",
        "whole_fruit": "whole_fruit_cup",
        "total_vegetable": "total_veg_cup",
        "green_and_bean": "greens_beans_cup",
        "total_protein": "protein_oz",
        "seafood_plant_protein": "seafood_plant_oz",
        "whole_grain": "whole_grains_oz",
        "dairy": "dairy_cup",
        "refined_grain": "refined_grains_oz",
        "sodium": "sodium_mg",
        "added_sugar": "added_sugars_g",
        "saturated_fat": "saturated_fat_g",
    }
    df = df.rename(columns=mapping)
    # convert units
    return df


def _prepare_2015(df):
    df = _prepare_common(df)
    df["added_sugars_g"] = df["added_sugars_g"] * 4  # teaspoons -> grams
    sat = df["saturated_fat_g"].astype(float)
    ratio = df["fatty_acid"].astype(float)
    df["monounsaturated_fat_g"] = ratio * sat / 2
    df["polyunsaturated_fat_g"] = ratio * sat / 2
    return df


def _prepare_2020(df):
    df = _prepare_common(df)
    energy = df["energy_kcal"]
    df["added_sugars_g"] = df["added_sugars_g"] / 100 * energy / 4
    sat = df["saturated_fat_g"].astype(float) / 100 * energy / 9
    df["sodium_mg"] = df["sodium_mg"] * 1000
    ratio = df["fatty_acid"].astype(float)
    df["saturated_fat_g"] = sat
    df["monounsaturated_fat_g"] = ratio * sat / 2
    df["polyunsaturated_fat_g"] = ratio * sat / 2
    return df


def _prepare_2020_v2(df):
    df = _prepare_common(df)
    df["added_sugars_g"] = df["added_sugars_g"] * 4
    sat = df["saturated_fat_g"].astype(float)
    ratio = df["fatty_acid"].astype(float)
    df["monounsaturated_fat_g"] = ratio * sat / 2
    df["polyunsaturated_fat_g"] = ratio * sat / 2
    return df


def test_hei2015_matches_reference():
    df = pd.read_csv("data/HEI2015_VALIDATION.csv")
    df = _prepare_2015(df)
    result = calculate_hei_2015(df)
    assert (
        pytest.approx(result.tolist(), rel=1e-3, abs=1e-3)
        == df["EXP_HEI2015_ALL"].tolist()
    )


def test_hei2020_matches_reference():
    df = pd.read_csv("data/HEI2020_VALIDATION.csv")
    df = _prepare_2020(df)
    adults = df[df["age"] >= 2]
    toddlers = df[df["age"] < 2]
    res_adult = calculate_hei_2020(adults)
    res_tod = calculate_hei_toddlers_2020(toddlers)
    result = pd.concat([res_adult, res_tod])
    expected = pd.concat([adults["EXP_HEI2020_ALL"], toddlers["EXP_HEI2020_ALL"]])
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == expected.tolist()


def test_hei2020_v2_matches_reference():
    df = pd.read_csv("data/HEI2020_V2_VALIDATION.csv")
    df = _prepare_2020_v2(df)
    adults = df[df["age"] >= 2]
    toddlers = df[df["age"] < 2]
    res_adult = calculate_hei_2020(adults)
    res_tod = calculate_hei_toddlers_2020(toddlers)
    result = pd.concat([res_adult, res_tod])
    expected = pd.concat([adults["EXP_HEI2020_ALL"], toddlers["EXP_HEI2020_ALL"]])
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == expected.tolist()
