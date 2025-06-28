# 🔬 Detailed Validation Guide

This document expands on the statistical foundations and scientific references used to verify each diet-quality index.
It is intended as a reference for researchers and future contributors who require a deep understanding of the algorithms.

---

## 1. Dietary Inflammatory Index (DII)

- **Reference Parameters**: 45 food parameters derived from Shivappa et al. (2014).
- **Computation Steps**:
  1. Standardize intakes against global means and standard deviations.
  2. Convert the standardized value to a percentile score.
  3. Center the percentile (`2 * pct - 1`).
  4. Multiply by the literature-derived inflammatory effect weight.
  5. Sum across all parameters for the final DII score.
- **Validation**: The integration tests compare computed scores to a curated reference table extracted from the original publication.

## 2. MIND Diet Score

- **Components**: 10 brain-healthy and 5 unhealthy food groups.
- **Scoring**: Each component contributes 0, 0.5, or 1 point based on weekly consumption thresholds.
- **Total Range**: 0–15.
- **Validation Strategy**: Synthetic datasets exercise edge cases (all healthy vs. all unhealthy) to verify that the score spans the expected range.

## 3. Healthy Eating Index 2015 (HEI‑2015)

- **Components**: 13 dietary elements measured per 1,000 kcal.
- **Method**: Density-based scoring with component‑specific cut points.
- **Total Range**: 0–100.
- **Validation**: Upcoming work will benchmark the Python implementation against the NCI SAS macros.

## 4. DASH Score

- **Components**: Fruits, vegetables, whole grains, low‑fat dairy, nuts/legumes, sodium, red/processed meats, and sweetened beverages.
- **Scoring**: Quintile ranking (reversed for moderation components) following Fung et al. (2008).
- **Range**: 8–40.
- **Validation**: Population quintiles from NHANES will be used to verify distributional properties.

---

## Cross‑validation Workflow

1. **Unit tests** confirm data validation and scoring outputs.
2. **Integration tests** run the FastAPI endpoints against known datasets.
3. **Statistical checks** compare summary statistics to those reported in the literature.
4. **Visual diagnostics** (histograms and boxplots) ensure each score follows the expected shape.

_This guide will evolve as additional datasets and language ports become available._
