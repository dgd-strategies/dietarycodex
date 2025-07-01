import tempfile
from pathlib import Path

import pandas as pd
from fastapi.testclient import TestClient

from compute.api import app
from compute.dii import DII_PARAMETER_KEYS

client = TestClient(app)


def test_score_endpoint_roundtrip(tmp_path):
    df = pd.DataFrame({key: [1.0, 2.0] for key in DII_PARAMETER_KEYS})
    file = tmp_path / "t.csv"
    df.to_csv(file, index=False)
    resp = client.post(
        "/score?indices=DII", files={"file": ("t.csv", file.read_bytes(), "text/csv")}
    )
    assert resp.status_code == 200
    data = resp.json()
    assert "filename" in data
    out = Path(tempfile.gettempdir()) / data["filename"]
    assert out.exists()
