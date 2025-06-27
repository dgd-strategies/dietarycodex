from fastapi import FastAPI, File, UploadFile, HTTPException
from fastapi.responses import FileResponse, JSONResponse
from fastapi.middleware.cors import CORSMiddleware
import pandas as pd
import io
import tempfile

# Import real scoring functions
from compute.dii import calculate_dii
from compute.mind import calculate_mind
from compute.hei import calculate_hei_2015
from compute.dash import calculate_dash

app = FastAPI(
    title="Dietary Index Web Calculator",
    description="Score multiple diet-quality indices from uploaded nutrition data.",
    version="0.1.0"
)

# Allow frontend access (adjust origins in production)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)

# Required columns based on your template
REQUIRED_COLS = [
    "id", "energy_kcal", "protein_g", "fat_g", "carb_g",
    "fiber_g", "vit_c_mg"
]

@app.post("/score")
async def score_diet_indices(file: UploadFile = File(...)):
    if not file.filename.endswith(".csv"):
        raise HTTPException(status_code=400, detail="Please upload a CSV file")

    content = await file.read()
    try:
        df = pd.read_csv(io.BytesIO(content))
    except Exception:
        raise HTTPException(status_code=400, detail="Unable to read CSV")

    # Check required headers
    missing = [col for col in REQUIRED_COLS if col not in df.columns]
    if missing:
        raise HTTPException(status_code=422, detail=f"Missing required columns: {missing}")

    try:
        # Apply real scoring functions
        df["DII"] = calculate_dii(df)
        df["MIND"] = calculate_mind(df)
        df["HEI_2015"] = calculate_hei_2015(df)
        df["DASH"] = calculate_dash(df)
    except Exception as e:
        raise HTTPException(status_code=500, detail=f"Scoring failed: {str(e)}")

    # Calculate summary stats
    stats = {}
    for col in ["DII", "MIND", "HEI_2015", "DASH"]:
        stats[col] = {
            "mean": round(df[col].mean(), 2),
            "std": round(df[col].std(), 2),
            "quintiles": df[col].quantile([0.2, 0.4, 0.6, 0.8]).round(2).tolist()
        }

    # Save result to a temporary file
    tmp = tempfile.NamedTemporaryFile(delete=False, suffix=".csv")
    df.to_csv(tmp.name, index=False)

    return JSONResponse({
        "message": "Success",
        "filename": tmp.name.split("/")[-1],
        "stats": stats
    })

@app.get("/download/{filename}")
def download_results(filename: str):
    try:
        return FileResponse(
            path=f"/tmp/{filename}",
            media_type="text/csv",
            filename=filename
        )
    except Exception:
        raise HTTPException(status_code=404, detail="File not found")

@app.get("/ping")
def ping():
    return {"status": "ok"}
