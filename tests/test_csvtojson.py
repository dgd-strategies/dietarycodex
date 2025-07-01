from pathlib import Path


def parse_js_style(text: str):
    lines = text.strip().splitlines()
    headers = [h.strip() for h in lines[0].split(",")]
    rows = []
    for line in lines[1:]:
        values = line.split(",")
        obj = {}
        for i, h in enumerate(headers):
            v = values[i].strip() if i < len(values) else ""
            if v == "":
                obj[h] = None
            else:
                try:
                    num = float(v)
                    if num.is_integer():
                        obj[h] = int(num)
                    else:
                        obj[h] = num
                except ValueError:
                    obj[h] = v
        rows.append(obj)
    return rows


def test_template_parses_alcohol():
    text = Path("data/template.csv").read_text()
    rows = parse_js_style(text)
    assert rows
    first = rows[0]
    assert "alcohol_g" in first
    assert first["alcohol_g"] == 100
