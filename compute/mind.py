import pandas as pd

def calculate_mind(df: pd.DataFrame) -> pd.Series:
    """
    Compute MIND diet score.
    """
    # Placeholder logic
    return (df["protein_g"] + df["fiber_g"]) / (df["fat_g"] + 1)