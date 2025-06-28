import json
from pathlib import Path

import pandas as pd


def test_template_csv_loads():
    csv_path = Path("assets/template.csv")
    df = pd.read_csv(csv_path)
    assert not df.empty


def test_template_json_valid():
    json_path = Path("assets/template.json")
    text = json_path.read_text()
    data = json.loads(text.replace("NaN", "null"))
    assert isinstance(data, list) and len(data) > 0
