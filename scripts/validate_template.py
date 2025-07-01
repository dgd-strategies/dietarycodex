import json
import sys
from pathlib import Path

import pandas as pd

PATH = "data/template.csv"


def main() -> None:
    sys.path.insert(0, str(Path(__file__).resolve().parents[1]))
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

    df = pd.read_csv(PATH)
    if "gender" not in df:
        df["gender"] = 1
    if "total_kcal" not in df:
        df["total_kcal"] = df.get("energy_kcal", 0)
    if "totalkcal_phdi" not in df:
        df["totalkcal_phdi"] = df.get("energy_kcal", 0)
    for col in REQUIRED_COLS:
        if col not in df:
            df[col] = 0
    results = {
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
    clean = {
        k: [None if pd.isna(x) else float(x) for x in v] for k, v in results.items()
    }
    print(json.dumps(clean))


if __name__ == "__main__":
    main()
