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

## 5. Alternative Healthy Eating Index (AHEI)

- **Components**: Vegetables, fruit, whole grains, nuts/legumes, omega‑3 fats, polyunsaturated fat, sugar‑sweetened beverages, red/processed meat, trans fat, sodium, and alcohol.
- **Scoring**: Linear scaling to 10 points per component with gender-specific cut points for whole grains and alcohol. Sodium is scored on cohort-specific deciles.
- **Range**: 0–110 when all components are summed.
- **Validation**: Reference results from the R `dietaryindex` package are included in `data/AHEI_VALIDATION.csv`.

### AHEI (serving-based variant, AHEIP)

- **Components**: Vegetables, whole fruit, white-to-red meat ratio, fiber, trans fat, polyunsaturated-to-saturated fat ratio, calcium, folate, and iron.
- **Range**: 0–90 when summed.
- **Validation**: Reference values from `data/AHEIP_VALIDATION.csv` ensure parity with the R implementation.
## 6. Alternate Mediterranean Diet Score (aMED)

- **Components**: Fruit, vegetables, whole grains, legumes, nuts, fish, red/processed meat, monounsaturated-to-saturated fat ratio, and alcohol.
- **Scoring**: Each component receives 1 point when the participant's intake is at or above the cohort median for healthy components, below the median for red/processed meat, and 1 point for 10–25 g of alcohol per day.
- **Range**: 0–9 when alcohol is included.
- **Validation**: Median-based logic ported from the R `dietaryindex` package.

## 7. MED Index in Serving Sizes (MEDI)

- **Components**: Olive oil, fruit, vegetables, legumes, nuts, fish/seafood, alcohol,
  sugar-sweetened beverages, sweets, discretionary fats, and red/processed meat.
- **Scoring**: Each component earns 1 point when the PREDIMED threshold is met; otherwise 0.
- **Range**: 0–11 (10 if alcohol is excluded).
- **Validation**: Ported directly from the R `dietaryindex` package.


## 8. Healthy Eating Index 2020

- **Components**: 13 dietary elements similar to HEI‑2015 but with updated cut points.
- **Variants**: Standard HEI‑2020 for adults and HEI‑Toddlers‑2020 for children aged 1–2 years.
- **Range**: 0–100.
- **Validation**: Thresholds and scoring logic are ported from the R `dietaryindex` package.

## 9. Planetary Health Diet Index (PHDI)

- **Origin**: Proposed by the EAT‑Lancet Commission for sustainable diets.
- **Scoring**: Components scaled from 0–10 (legumes and soy max 5) with gender‑specific cut points for whole grains.
- **Range**: 0–140 when summed across all components.
- **Validation**: Implemented according to the R `dietaryindex` package which
  serves as the reference implementation. A small sample dataset
  (`PHDI_VALIDATION.csv`) is included in this repository for automated
  cross‑checks.

## 10. American Cancer Society 2020 Diet Score

- **Variants**: `ACS2020_V1` using the percent of calories from highly processed foods and refined grains, and `ACS2020_V2` using servings per 1000 kcal.
- **Scoring**: Components are ranked by gender-specific quartiles. Healthy foods earn up to 0.75 or 3 points depending on the component, while unhealthy foods are reverse scored. Sugar-sweetened beverages have fixed cut points.
- **Range**: 0–12 when all components are summed.
- **Validation**: The Python functions mirror the R implementations in the `dietaryindex` package. Example validation datasets (`ACS2020_V1_validation.csv`, `ACS2020_V2_validation.csv`) are included in the repository.

---

## Cross‑validation Workflow

1. **Unit tests** confirm data validation and scoring outputs.
2. **Integration tests** run the FastAPI endpoints against known datasets.
3. **Statistical checks** compare summary statistics to those reported in the literature.
4. **Visual diagnostics** (histograms and boxplots) ensure each score follows the expected shape.

_This guide will evolve as additional datasets and language ports become available._
