# 🔬 Detailed Validation Guide

This document expands on the statistical foundations and scientific references used to verify each diet-quality index.
It is intended as a reference for researchers and future contributors who require a deep understanding of the algorithms.
The algorithms are ported from the [`dietaryindex` R package](https://jamesjiadazhan.github.io/dietaryindex_manual/index.html), which provides example datasets used here for cross-validation.


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
- **References**: Shivappa N et al. Public Health Nutr. 2014; Hébert JR et al. Adv Nutr. 2019.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#dii)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_dii), [ASA24](excel_serving_sizes_part2.md#asa24_dii), [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_dii)


## 2. MIND Diet Score

- **Components**: 10 brain-healthy and 5 unhealthy food groups.
- **Scoring**: Each component contributes 0, 0.5, or 1 point based on weekly consumption thresholds.
- **Total Range**: 0–15.
- **References**: Morris MC et al. Alzheimers Dement. 2015.
- **Validation Strategy**: Synthetic datasets exercise edge cases (all healthy vs. all unhealthy) to verify that the score spans the expected range.

## 3. Healthy Eating Index 2015 (HEI‑2015)

- **Components**: 13 dietary elements measured per 1,000 kcal.
- **Method**: Density-based scoring with component‑specific cut points.
- **References**: Krebs-Smith SM et al. J Acad Nutr Diet. 2018.
- **Total Range**: 0–100.
- **Validation**: Reference results from the R `dietaryindex` package are included
  in `data/HEI2015_VALIDATION.csv`, `data/HEI2020_VALIDATION.csv`, and
  `data/HEI2020_V2_VALIDATION.csv`.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#hei2015)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_hei2015), [ASA24](excel_serving_sizes_part2.md#asa24_hei2015), [DHQ3](excel_serving_sizes_part2.md#dhq3_hei2015), [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_hei2015)


## 4. DASH Score

- **Components**: Fruits, vegetables, whole grains, low‑fat dairy, nuts/legumes, sodium, red/processed meats, and sweetened beverages.
- **References**: Fung TT et al. JAMA. 2008.
- **Scoring**: Quintile ranking (reversed for moderation components) following Fung et al. (2008).
- **Range**: 8–40.
- **Validation**: Population quintiles from NHANES will be used to verify distributional properties.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#dash)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_dash), [ASA24](excel_serving_sizes_part2.md#asa24_dash), [DHQ3](excel_serving_sizes_part3.md#dhq3_dash), [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_dash)


## 5. Alternative Healthy Eating Index (AHEI)

- **References**: Chiuve SE et al. J Nutr. 2012.
- **Components**: Vegetables, fruit, whole grains, nuts/legumes, omega‑3 fats, polyunsaturated fat, sugar‑sweetened beverages, red/processed meat, trans fat, sodium, and alcohol.
- **Scoring**: Linear scaling to 10 points per component with gender-specific cut points for whole grains and alcohol. Sodium is scored on cohort-specific deciles.
- **Range**: 0–110 when all components are summed.
- **Validation**: Reference results from the R `dietaryindex` package are included in `data/AHEI_VALIDATION.csv`.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#ahei-2010)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_ahei-2010), [ASA24](excel_serving_sizes_part2.md#asa24_ahei-2010), [DHQ3](excel_serving_sizes_part2.md#dhq3_ahei-2010), [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_ahei-2010)


- **References**: McCullough ML et al. AJCN. 2002.
### AHEI (serving-based variant, AHEIP)

- **Components**: Vegetables, whole fruit, white-to-red meat ratio, fiber, trans fat, polyunsaturated-to-saturated fat ratio, calcium, folate, and iron.
- **Range**: 0–90 when summed.
- **Validation**: Reference values from `data/AHEIP_VALIDATION.csv` ensure parity with the R implementation.
- **References**: Fung TT et al. Arch Intern Med. 2009.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#aheip)
  - Serving sizes: [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_aheip)

## 6. Alternate Mediterranean Diet Score (aMED)

- **Components**: Fruit, vegetables, whole grains, legumes, nuts, fish, red/processed meat, monounsaturated-to-saturated fat ratio, and alcohol.
- **Scoring**: Each component receives 1 point when the participant's intake is at or above the cohort median for healthy components, below the median for red/processed meat, and 1 point for 10–25 g of alcohol per day.
- **Range**: 0–9 when alcohol is included.
- **Validation**: Median-based logic ported from the R `dietaryindex` package.

- **References**: Estruch R et al. N Engl J Med. 2013.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#med)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_med), [ASA24](excel_serving_sizes_part2.md#asa24_med), [DHQ3](excel_serving_sizes_part3.md#dhq3_med), [BLOCK FFQ](excel_serving_sizes_part3.md#block_ffq_med)

## 7. MED Index in Serving Sizes (MEDI)

- **Components**: Olive oil, fruit, vegetables, legumes, nuts, fish/seafood, alcohol,
  sugar-sweetened beverages, sweets, discretionary fats, and red/processed meat.
- **Scoring**: Each component earns 1 point when the PREDIMED threshold is met; otherwise 0.
- **Range**: 0–11 (10 if alcohol is excluded).
- **Validation**: Ported directly from the R `dietaryindex` package.
- **References**: U.S. Department of Agriculture HEI‑2020 Documentation.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#medi)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_medi)



## 8. Healthy Eating Index 2020

- **Components**: 13 dietary elements similar to HEI‑2015 but with updated cut points.
- **Variants**: Standard HEI‑2020 for adults and HEI‑Toddlers‑2020 for children aged 1–2 years.
- **Range**: 0–100.
- **Validation**: Thresholds and scoring logic are ported from the R `dietaryindex` package.
- **References**: U.S. Department of Agriculture HEI‑2020 documentation.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#hei2020)
  - Serving sizes: [NHANES](excel_serving_sizes_part1.md#nhanes_hei2020), [ASA24](excel_serving_sizes_part2.md#asa24_hei2020)


## 9. Planetary Health Diet Index (PHDI)

- **Origin**: Proposed by the EAT‑Lancet Commission for sustainable diets.
- **Scoring**: Components scaled from 0–10 (legumes and soy max 5) with gender‑specific cut points for whole grains.
- **Range**: 0–140 when summed across all components.
- **Validation**: Implemented according to the R `dietaryindex` package which
- **References**: Cacau LT et al. Nutrients. 2023; EAT‑Lancet Commission.
  serves as the reference implementation. A small sample dataset
  (`PHDI_VALIDATION.csv`) is included in this repository for automated
  cross‑checks.
- **Tables**:
  - [Scoring Algorithm](excel_scoring_algorithm.md#phdi)


## 10. American Cancer Society 2020 Diet Score

- **Variants**: `ACS2020_V1` using the percent of calories from highly processed foods and refined grains, and `ACS2020_V2` using servings per 1000 kcal.
- **Scoring**: Components are ranked by gender-specific quartiles. Healthy foods earn up to 0.75 or 3 points depending on the component, while unhealthy foods are reverse scored. Sugar-sweetened beverages have fixed cut points.
- **Range**: 0–12 when all components are summed.
- **Validation**: The Python functions mirror the R implementations in the `dietaryindex` package. Example validation datasets (`ACS2020_V1_validation.csv`, `ACS2020_V2_validation.csv`) are included in the repository.
- **References**: Kushi LH et al. CA Cancer J Clin. 2020.
- **Tables**:
  - [ACS2020_V1 Algorithm](excel_scoring_algorithm.md#acs-2020_v1)
  - [ACS2020_V2 Algorithm](excel_scoring_algorithm.md#acs-2020_v2)


---

## Cross‑validation Workflow

1. **Unit tests** confirm data validation and scoring outputs.
2. **Integration tests** run the FastAPI endpoints against known datasets.
3. **Statistical checks** compare summary statistics to those reported in the literature.
4. **Visual diagnostics** (histograms and boxplots) ensure each score follows the expected shape.
5. **Pre-commit**: Running `pre-commit run --all-files` executes these tests
   automatically before code is committed. Ensure all requirements are installed
   first with `pip install -r requirements.txt`.
## Spreadsheet Resources

Full tables extracted from the provided Excel files are available at [excel_scoring_algorithm.md](excel_scoring_algorithm.md) and [excel_serving_sizes_part1.md), [excel_serving_sizes_part2.md](excel_serving_sizes_part2.md), [excel_serving_sizes_part3.md](excel_serving_sizes_part3.md).


_This guide will evolve as additional datasets and language ports become available._
