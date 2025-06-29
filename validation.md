# 🧪 Validation & Scoring Standards

This page acts as a quick reference for the validation datasets and key calculation notes used throughout the project. Each Python function mirrors the R `dietaryindex` package. The tables below ensure the Codex agent knows the expected ranges and sources when writing code.

## Dietary Inflammatory Index (DII)
- **Range:** −8.87 to +7.98
- **Units:** dimensionless score
- **Dataset:** `DII_validation_result.csv`
- **Reference:** Shivappa N. *Public Health Nutr.* 2014

## MIND Diet
- **Range:** 0–15
- **Units:** component points (0, 0.5, 1)
- **Dataset:** synthetic edge-case CSVs
- **Reference:** Morris MC. *Alzheimers Dement.* 2015

## Healthy Eating Index 2015
- **Range:** 0–100
- **Units:** points per 1,000 kcal
- **Dataset:** `HEI2015_VALIDATION.csv`
- **Reference:** Krebs‑Smith SM. *J Acad Nutr Diet.* 2018

## DASH Score
- **Range:** 8–40
- **Units:** quintile scores
- **Dataset:** `DASH_VALIDATION.csv`
- **Reference:** Fung TT. *JAMA.* 2008

## Alternative Healthy Eating Index (AHEI)
- **Range:** 0–110
- **Units:** 11 components × 10
- **Dataset:** `AHEI_VALIDATION.csv`
- **Reference:** Chiuve SE. *J Nutr.* 2012

## AHEI Serving Variant (AHEIP)
- **Range:** 0–90
- **Units:** 9 components × 10
- **Dataset:** `AHEIP_VALIDATION.csv`
- **Reference:** Fung TT. *Arch Intern Med.* 2009

## Alternate Mediterranean Diet (aMED)
- **Range:** 0–9
- **Units:** binary component scores
- **Dataset:** `MED_VALIDATION.csv`
- **Reference:** Fung TT. *Arch Intern Med.* 2009

## MED Index in Servings (MEDI)
- **Range:** 0–11
- **Units:** binary component scores
- **Dataset:** `MEDI_V2_validation.csv`
- **Reference:** Estruch R. *N Engl J Med.* 2013

## Healthy Eating Index 2020
- **Range:** 0–100
- **Units:** points per 1,000 kcal
- **Dataset:** `HEI2020_VALIDATION.csv`
- **Reference:** USDA CNPP documentation

## Planetary Health Diet Index (PHDI)
- **Range:** 0–140
- **Units:** 14 components × 10
- **Dataset:** `PHDI_VALIDATION.csv`
- **Reference:** Cacau LT. *Nutrients.* 2023

## American Cancer Society 2020 Score
- **Range:** 0–12
- **Variants:** V1 (percent calories), V2 (servings/1,000 kcal)
- **Datasets:** `ACS2020_V1_validation.csv`, `ACS2020_V2_validation.csv`
- **Reference:** Kushi LH. *CA Cancer J Clin.* 2020

## Validation Workflow
1. Unit tests compare each score to the datasets above.
2. Statistical checks ensure distributions match published examples.
3. Integration tests exercise the FastAPI endpoints.
4. `pre-commit run --all-files` executes these checks automatically.

_Last updated: 2025‑06‑29_
