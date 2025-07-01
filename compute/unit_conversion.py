import json
from pathlib import Path
from typing import Dict, Tuple

import pandas as pd

BASE_DIR = Path(__file__).resolve().parent.parent
DEFAULT_UNITS_PATH = BASE_DIR / "schema" / "default_units.json"
DEFAULT_UNITS: Dict[str, str] = json.loads(DEFAULT_UNITS_PATH.read_text())

_UNIT_SCALE: Dict[Tuple[str, str], float] = {
    ("g", "mg"): 1000,
    ("mg", "g"): 0.001,
    ("mcg", "mg"): 0.001,
    ("mg", "mcg"): 1000,
    ("g", "mcg"): 1_000_000,
    ("mcg", "g"): 0.000001,
    ("kj", "kcal"): 1 / 4.184,
    ("kcal", "kj"): 4.184,
}


def infer_units(df: pd.DataFrame) -> Dict[str, str]:
    units: Dict[str, str] = {}
    rename: Dict[str, str] = {}
    for col in df.columns:
        unit = None
        canonical = col
        lower = col.lower()
        for suffix in ("_mg", "_mcg", "_g", "_kcal", "_kj"):
            if lower.endswith(suffix):
                unit = suffix[1:]
                canonical = col[: -len(suffix)]
                break
        units[canonical] = unit or DEFAULT_UNITS.get(canonical, "")
        rename[col] = canonical
    df.rename(columns=rename, inplace=True)
    return units


def convert_to_canonical_units(df: pd.DataFrame, units: Dict[str, str]) -> pd.DataFrame:
    for col, unit in units.items():
        expected = DEFAULT_UNITS.get(col)
        if expected and unit and unit != expected:
            factor = _UNIT_SCALE.get((unit, expected))
            if factor is not None:
                df[col] = pd.to_numeric(df[col], errors="coerce") * factor
    return df


def rename_for_scoring(df: pd.DataFrame) -> pd.DataFrame:
    rename = {
        col: f"{col}_{DEFAULT_UNITS[col]}" for col in df.columns if col in DEFAULT_UNITS
    }
    return df.rename(columns=rename)
