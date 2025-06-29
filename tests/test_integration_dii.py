# tests/test_integration_dii.py
import io
import json
from pathlib import Path

import pandas as pd
import pytest
from fastapi.testclient import TestClient

from compute.api import app
from compute.dii import calculate_dii

# Initialize TestClient for the FastAPI app
client = TestClient(app)

# Paths (adjust if you move the CSV)
ROOT = Path(__file__).parent.parent
DETAILED_CSV = ROOT / "data" / "detailed_dii_table_20240606.csv"
PARAMS_JSON = ROOT / "data" / "dii_parameters.json"

# Load the reference table and normalize column names
df_ref = pd.read_csv(DETAILED_CSV)
df_ref.columns = df_ref.columns.str.strip()

# Load the DII parameters (list of dicts with "name", etc.)
params = json.loads(PARAMS_JSON.read_text())

# Some parameter names don’t match CSV exactly—override here:
MAPPING_OVERRIDES = {
    "b-carotene": "Beta-carotene",
    "fibre": "Fiber",
}


def build_input_df():
    """
    From the detailed ref table, extract only the raw nutrient columns
    and rename them to exactly match the parameter names used by calculate_dii().
    """
    mapping = {}
    for p in params:
        pname = p["name"]
        key = pname.lower()
        target = MAPPING_OVERRIDES.get(key, key)
        # find the CSV column with a case-insensitive match
        col = next(c for c in df_ref.columns if c.strip().lower() == target)
        mapping[col] = pname

    df_input = df_ref[list(mapping.keys())].rename(columns=mapping)
    return df_input


def test_calculate_dii_matches_reference():
    df_input = build_input_df()
    result = calculate_dii(df_input)
    expected = df_ref["score_dii"]
    # Element-wise comparison within tolerance
    assert pytest.approx(result.tolist(), rel=1e-3, abs=1e-3) == expected.tolist()


def test_full_stack_score_and_download(tmp_path):
    df_input = build_input_df()
    # 1) POST /score
    response = client.post(
        "/score",
        files={"file": ("test.csv", df_input.to_csv(index=False).encode(), "text/csv")},
    )
    assert response.status_code == 200
    data = response.json()
    # 2) Check JSON stats for DII
    d_stats = data["stats"]["DII"]
    series = calculate_dii(df_input)
    assert abs(d_stats["mean"] - series.mean()) < 1e-2
    assert abs(d_stats["std"] - series.std()) < 1e-2
    assert abs(d_stats["min"] - series.min()) < 1e-2
    assert abs(d_stats["max"] - series.max()) < 1e-2

    # 3) GET /download and verify CSV contents
    filename = data["filename"]
    dl = client.get(f"/download/{filename}")
    assert dl.status_code == 200
    df_out = pd.read_csv(io.BytesIO(dl.content))
    # The output CSV should have a column "DII" matching the reference scores
    assert pytest.approx(df_out["DII"].tolist(), rel=1e-3, abs=1e-3) == series.tolist()
