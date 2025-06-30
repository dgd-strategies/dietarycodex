import pandas as pd

from compute.ahei import AHEI_COMPONENT_KEYS, calculate_ahei
from compute.dash import DASH_COMPONENT_KEYS, calculate_dash


def test_output_variance_dash_ahei():
    n = 5
    dash_df = pd.DataFrame({c: range(1, n + 1) for c in DASH_COMPONENT_KEYS})
    dash_scores = calculate_dash(dash_df)
    assert dash_scores.nunique() > 1

    cols = {c: range(1, n + 1) for c in AHEI_COMPONENT_KEYS}
    cols["gender"] = [1] * n
    ahei_df = pd.DataFrame(cols)
    ahei_scores = calculate_ahei(ahei_df)
    assert ahei_scores.nunique() > 1
