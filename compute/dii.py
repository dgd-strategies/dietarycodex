import math

import numpy as np
import pandas as pd

from compute.base import load_parameters, validate_dataframe

# Load DII parameters from JSON
dii_params = load_parameters("dii_parameters.json")
# Expose parameter keys for API validation
DII_PARAMETER_KEYS = [p["name"] for p in dii_params]


def get_dii_parameters() -> list:
    """Return the loaded DII parameter definitions."""
    return dii_params


def calculate_dii(df: pd.DataFrame) -> pd.Series:
    """
    Calculate Dietary Inflammatory Index (DII) dynamically from parameters.

    Uses global means, SDs, and effect scores defined in dii_parameters.json.

    Steps for each parameter:
      1. Standardize: z = (X - mean) / sd
      2. Percentile rank: pct = rankpct(z)
      3. Center percentile: cp = 2*pct - 1
      4. Weighted sum: Σ(cp * effect)

    Args:
        df: pandas DataFrame with nutrient/food parameter columns.
    Returns:
        pandas Series of DII scores per row.
    """
    # Validate that input contains expected numeric columns\
    validate_dataframe(df, DII_PARAMETER_KEYS)

    # Initialize score series
    score = pd.Series(0.0, index=df.index)

    # Compute for each parameter
    for param in dii_params:
        name = param["name"]
        mean = param.get("mean")
        sd = param.get("sd")
        effect = param.get("effect")

        x = df[name].fillna(mean)
        z = (x - mean) / sd
        pct = 0.5 * (1 + np.vectorize(math.erf)(z / math.sqrt(2)))
        cp = 2 * pct - 1
        score += cp * effect

    return score
