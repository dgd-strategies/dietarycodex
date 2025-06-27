import pandas as pd

def calculate_dash(df: pd.DataFrame) -> pd.Series:
    """
    Compute DASH score.
    """
    # Placeholder — fake logic based on nutrient ratios
    return df["fiber_g"] + df["vit_c_mg"] / 20