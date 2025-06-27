import pandas as pd

def calculate_hei_2015(df: pd.DataFrame) -> pd.Series:
    """
    Compute Healthy Eating Index 2015 (HEI-2015).
    """
    # Placeholder — substitute with real component scoring
    return df["fiber_g"] * 2 + df["vit_c_mg"] / 10