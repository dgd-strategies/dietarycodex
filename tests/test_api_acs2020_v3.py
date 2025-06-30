from unittest.mock import patch

import pandas as pd
from fastapi.testclient import TestClient

from compute.acs2020 import ACS2020_V3_KEYS
from compute.api import app

client = TestClient(app)


def make_df(n=2):
    df = pd.DataFrame({col: [1] * n for col in ACS2020_V3_KEYS})
    df["gender"] = 1
    df["TOTALKCAL_ACS2020"] = 2000
    return df


def test_acs2020_v3_invoked(tmp_path):
    df = make_df()
    with patch("compute.api.calculate_acs2020_v3") as mock_calc:
        mock_calc.return_value = pd.Series([1] * len(df))
        response = client.post(
            "/score?indices=ACS2020_V3",
            files={"file": ("test.csv", df.to_csv(index=False), "text/csv")},
        )
        assert response.status_code == 200
        mock_calc.assert_called_once()
