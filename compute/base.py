import json
from pathlib import Path

import pandas as pd


def load_parameters(filename: str) -> list:
    """
    Load a JSON parameter file from the compute directory.

    Args:
        filename: Name of the JSON file (e.g., 'dii_parameters.json').
    Returns:
        A list of parameter definitions loaded from JSON.
    """
    path = Path(__file__).parent / filename
    if not path.exists():
        raise FileNotFoundError(f"Parameters file not found: {path}")
    return json.loads(path.read_text())


def validate_dataframe(df: pd.DataFrame, required_cols: list) -> bool:
    """
    Ensure that the DataFrame contains all required columns and numeric data.

    Args:
        df: pandas DataFrame to validate.
        required_cols: List of column names that must be present.
    Returns:
        True if validation passes; otherwise raises ValueError.
    """
    missing = [c for c in required_cols if c not in df.columns]
    if missing:
        raise ValueError(f"Missing required columns: {missing}")

    for col in df.columns:
        if col == "id":
            continue
        if not pd.api.types.is_numeric_dtype(df[col]):
            raise ValueError(f"Column '{col}' must be numeric")

    return True


def compute_summary_stats(df: pd.DataFrame, cols: list) -> dict:
    """
    Compute summary statistics for specified columns in the DataFrame.

    Args:
        df: pandas DataFrame with score columns.
        cols: List of column names for which to calculate stats.
    Returns:
        Dictionary mapping each column name to its summary stats.
    """
    stats = {}
    for col in cols:
        s = df[col]
        stats[col] = {
            "mean": round(s.mean(), 2),
            "std": round(s.std(), 2),
            "min": round(s.min(), 2),
            "max": round(s.max(), 2),
            "median": round(s.median(), 2),
            "quintiles": s.quantile([0.2, 0.4, 0.6, 0.8]).round(2).tolist(),
        }
    return stats
