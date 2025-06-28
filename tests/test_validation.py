import pandas as pd

from compute.base import validate_dataframe
from compute.dii import DII_PARAMETER_KEYS


def test_validate_coerces_numeric_and_allows_nan():
    cols = DII_PARAMETER_KEYS[:3]  # pick a few columns
    data = [
        {cols[0]: "1.5", cols[1]: "", cols[2]: "2"},
        {cols[0]: "3", cols[1]: "4", cols[2]: None},
    ]
    df = pd.DataFrame(data)
    # Should not raise despite strings and blanks
    assert validate_dataframe(df, cols)
    assert df[cols[0]].dtype.kind in "fiu"
    assert df[cols[1]].isna().all() or df[cols[1]].dtype.kind in "fiu"
