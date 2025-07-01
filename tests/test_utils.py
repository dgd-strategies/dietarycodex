import pandas as pd
import pytest
from fastapi.testclient import TestClient

from compute.api import app
from compute.base import compute_summary_stats
from compute.hcns import HCNS_MIND_MAP, aggregate_hcns_to_mind
from compute.mapping import apply_mapping
from compute.mind import MIND_COMPONENT_KEYS

client = TestClient(app)


def test_ping_endpoint():
    resp = client.get("/ping")
    assert resp.status_code == 200
    assert resp.json() == {"status": "ok"}


def test_apply_mapping_renames_columns():
    df = pd.DataFrame({"A": [1], "B": [2], "C": [3]})
    mapping = {"A": "X", "B": "Y"}
    out = apply_mapping(df, mapping)
    assert list(out.columns) == ["X", "Y", "C"]
    assert out["X"].iloc[0] == 1
    assert out["Y"].iloc[0] == 2


def test_compute_summary_stats_basic():
    df = pd.DataFrame({"col": [1, 2, 3, 4]})
    stats = compute_summary_stats(df, ["col"])
    expected = {
        "col": {
            "mean": 2.5,
            "std": round(df["col"].std(), 2),
            "min": 1.0,
            "max": 4.0,
            "median": 2.5,
            "quintiles": df["col"].quantile([0.2, 0.4, 0.6, 0.8]).round(2).tolist(),
        }
    }
    assert stats == expected


def test_aggregate_hcns_to_mind_basic():
    data = {col: [1, 1] for cols in HCNS_MIND_MAP.values() for col in cols}
    df = pd.DataFrame(data)
    result = aggregate_hcns_to_mind(df)
    assert list(result.columns) == MIND_COMPONENT_KEYS
    for key in MIND_COMPONENT_KEYS:
        if key == "olive_oil_daily_use":
            assert (result[key] == 1).all()
        else:
            assert (result[key] == len(HCNS_MIND_MAP[key])).all()


def test_aggregate_hcns_to_mind_missing():
    data = {col: [1] for cols in HCNS_MIND_MAP.values() for col in cols}
    missing_col = next(iter(HCNS_MIND_MAP["fried_food_servings"]))
    data.pop(missing_col)
    df = pd.DataFrame(data)
    with pytest.raises(ValueError):
        aggregate_hcns_to_mind(df)
