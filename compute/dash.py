import pandas as pd

from compute.base import validate_dataframe

# DASH components and scoring direction (Fung et al., 2008)
# 'positive' => higher intake is better; 'reverse' => lower intake is better
# df components should match these names exactly when validating
# df parameters define grams or mg as listed below
_DASH_COMPONENTS = [
    {"name": "fruits", "type": "positive"},
    {"name": "vegetables", "type": "positive"},
    {"name": "whole_grains", "type": "positive"},
    {"name": "low_fat_dairy", "type": "positive"},
    {"name": "nuts_legumes", "type": "positive"},
    {"name": "sodium", "type": "reverse"},
    {"name": "red_processed_meats", "type": "reverse"},
    {"name": "sweetened_beverages", "type": "reverse"},
]

# Expose component keys for API validation
# df parameter names for validation
DASH_COMPONENT_KEYS = [comp["name"] for comp in _DASH_COMPONENTS]


def calculate_dash(df: pd.DataFrame) -> pd.Series:
    """
    Calculate DASH diet score based on Fung et al. (2008) methodology:
    - Eight components scored on cohort-specific quintiles (1 to 5)
    - Positive components: highest quintile = 5 points
    - Reverse components: lowest quintile = 5 points
    Total DASH score range: 8 (lowest adherence) to 40 (highest adherence).

    Args:
        df: pandas DataFrame with required DASH component columns.
    Returns:
        pandas Series of DASH scores per row.
    """
    # Validate that required columns exist and are numeric
    validate_dataframe(df, DASH_COMPONENT_KEYS)

    # Prepare a DataFrame to hold quintile ranks
    ranks = pd.DataFrame(index=df.index)

    for comp in _DASH_COMPONENTS:
        col = comp["name"]

        # Compute cohort quintiles (1-5)
        try:
            ranks[col] = pd.qcut(df[col], 5, labels=False, duplicates="drop") + 1
        except (ValueError, IndexError):
            # Fallback to percentile-based ranking or empty data handling
            scores = (df[col].rank(pct=True) * 5).clip(1, 5).round()
            ranks[col] = scores.fillna(3).astype(int)

        # Reverse scoring if needed
        if comp["type"] == "reverse":
            ranks[col] = 6 - ranks[col]

    # Sum component scores for total DASH
    dash_score = ranks.sum(axis=1)
    return dash_score
