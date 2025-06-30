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
    results = {
        "DII": calculate_dii(df).tolist(),
        "DASH": calculate_dash(df).tolist(),
        "DASHI": calculate_dashi(df).tolist(),
        "AHEI": calculate_ahei(df[AHEI_COMPONENT_KEYS]).tolist(),
        "AHEIP": calculate_aheip(df[AHEIP_COMPONENT_KEYS]).tolist(),
        "AMED": calculate_amed(df).tolist(),
        "MEDI": calculate_medi(df).tolist(),
        "MEDI_V2": calculate_medi_v2(df).tolist(),
        "HEI_2015": calculate_hei_2015(df).tolist(),
        "HEI_2020": calculate_hei_2020(df).tolist(),
        "HEI_TODDLERS_2020": calculate_hei_toddlers_2020(df).tolist(),
        "PHDI": calculate_phdi(df).tolist(),
        "PHDI_V2": calculate_phdi_v2(df).tolist(),
        "ACS2020_V1": calculate_acs2020_v1(df).tolist(),
        "ACS2020_V2": calculate_acs2020_v2(df).tolist(),
    }
    print(json.dumps(results))


if __name__ == "__main__":
    main()
