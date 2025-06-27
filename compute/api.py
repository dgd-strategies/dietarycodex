import io
import logging
import tempfile
from pathlib import Path
from typing import List
from uuid import uuid4

import pandas as pd
from fastapi import FastAPI, File, HTTPException, Query, UploadFile
from fastapi.middleware.cors import CORSMiddleware
from fastapi.responses import JSONResponse, StreamingResponse

from compute.dash import DASH_COMPONENT_KEYS, calculate_dash
from compute.dii import calculate_dii, get_dii_parameters
from compute.hei import HEI_COMPONENT_KEYS, calculate_hei_2015
from compute.mind import MIND_COMPONENT_KEYS, calculate_mind

logging.basicConfig(level=logging.INFO)
logger = logging.getLogger(__name__)

# Core required columns
CORE_COLS = [
    "id",
    "energy_kcal",
    "protein_g",
    "fat_g",
    "carb_g",
    "fiber_g",
    "vit_c_mg",
]

# Dynamically build full required columns list for validation
REQUIRED_COLS = (
    CORE_COLS
    + [p["name"] for p in get_dii_parameters()]
    + HEI_COMPONENT_KEYS
    + MIND_COMPONENT_KEYS
    + DASH_COMPONENT_KEYS
)

app = FastAPI(
    title="Dietary Index Web Calculator",
    description=("Score multiple diet-quality indices from uploaded nutrition data."),
    version="0.1.0",
)

# Enable CORS for local dev; lock down origins in production
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


def check_core_columns(df: pd.DataFrame):
    missing = [c for c in CORE_COLS if c not in df.columns]
    if missing:
        raise HTTPException(
            status_code=422,
            detail=f"Missing core columns: {missing}",
        )


@app.post("/score")
async def score_diet_indices(
    file: UploadFile = File(...),
    indices: List[str] = Query(
        default=["DII"],
        description="Which indices to compute",
    ),
):
    # Validate file
    if not file.filename or not file.filename.lower().endswith(".csv"):
        raise HTTPException(status_code=400, detail="Upload a valid CSV file")
    logger.info("Received file: %s", file.filename)

    # Read CSV
    content = await file.read()
    try:
        df = pd.read_csv(io.BytesIO(content), encoding="utf-8-sig")
    except Exception as e:
        logger.error("CSV parse error: %s", e)
        raise HTTPException(status_code=400, detail="Unable to read CSV file")

    # Basic shape check (optional core columns for testing)

    # Compute selected indices
    results = {}
    if "DII" in indices:
        logger.info("Computing DII...")
        results["DII"] = calculate_dii(df)
    if "MIND" in indices:
        logger.info("Computing MIND...")
        results["MIND"] = calculate_mind(df)
    if "HEI_2015" in indices:
        logger.info("Computing HEI-2015...")
        results["HEI_2015"] = calculate_hei_2015(df)
    if "DASH" in indices:
        logger.info("Computing DASH...")
        results["DASH"] = calculate_dash(df)

    # Attach results to DataFrame
    for name, series in results.items():
        df[name] = series

    # Summary statistics
    stats = {}
    for name, series in results.items():
        stats[name] = {
            "mean": float(series.mean()),
            "std": float(series.std()),
            "min": float(series.min()),
            "max": float(series.max()),
            "median": float(series.median()),
            "quintiles": series.quantile([0.2, 0.4, 0.6, 0.8]).tolist(),
        }

    # Add API version column
    df["api_version"] = app.version

    # Persist scored CSV to a temp file for later download
    filename = f"results_{uuid4().hex}.csv"
    output_path = Path(tempfile.gettempdir()) / filename
    df.to_csv(output_path, index=False)

    return JSONResponse({"message": "Success", "filename": filename, "stats": stats})


@app.get("/download/{filename}")
def download_scored_csv(filename: str):
    path = Path(tempfile.gettempdir()) / filename
    if not path.exists():
        raise HTTPException(status_code=404, detail="File not found")

    return StreamingResponse(
        path.open("rb"),
        media_type="text/csv",
        headers={"Content-Disposition": f"attachment; filename={filename}"},
    )


@app.get("/ping")
def ping():
    return {"status": "ok"}
