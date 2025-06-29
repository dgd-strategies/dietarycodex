# Scoring Algorithms

This page summarizes the scoring cut points for each diet-quality index implemented in this project. The tables below are simplified references derived from the official scoring manuals.

## HEI-2020

| Component | Standard for Max | Standard for Min | Max Points |
|-----------|-----------------|-----------------|-----------|
| Total fruit | ≥0.8 cup eq./1,000 kcal | No fruit | 5 |
| Whole fruit | ≥0.4 cup eq./1,000 kcal | No whole fruit | 5 |
| Total vegetables | ≥1.1 cup eq./1,000 kcal | No vegetables | 5 |
| Greens & beans | ≥0.2 cup eq./1,000 kcal | No dark-green veg or legumes | 5 |
| Whole grains | ≥1.5 oz eq./1,000 kcal | No whole grains | 10 |
| Dairy | ≥1.3 cup eq./1,000 kcal | No dairy | 10 |
| Total protein foods | ≥2.5 oz eq./1,000 kcal | No protein foods | 5 |
| Seafood & plant protein | ≥0.8 oz eq./1,000 kcal | No seafood or plant proteins | 5 |
| Fatty acids *(MUFA+PUFA)/SFA* | ≥2.5 | ≤1.2 | 10 |
| Refined grains | ≤1.8 oz eq./1,000 kcal | ≥4.3 oz eq./1,000 kcal | 10 |
| Sodium | ≤1.1 g/1,000 kcal | ≥2.0 g/1,000 kcal | 10 |
| Added sugars | ≤6.5% of energy | ≥26% of energy | 10 |
| Saturated fat | ≤8% of energy | ≥16% of energy | 10 |

## HEI‑Toddlers‑2020

| Component | Standard for Max | Standard for Min | Max Points |
|-----------|-----------------|-----------------|-----------|
| Total fruit | ≥0.7 cup eq./1,000 kcal | No fruit | 5 |
| Whole fruit | ≥0.3 cup eq./1,000 kcal | No whole fruit | 5 |
| Total vegetables | ≥0.9 cup eq./1,000 kcal | No vegetables | 5 |
| Greens & beans | ≥0.1 cup eq./1,000 kcal | No dark-green veg or legumes | 5 |
| Whole grains | ≥1.5 oz eq./1,000 kcal | No whole grains | 10 |
| Dairy | ≥2.0 cup eq./1,000 kcal | No dairy | 10 |
| Total protein foods | ≥2.0 oz eq./1,000 kcal | No protein foods | 5 |
| Seafood & plant protein | ≥0.5 oz eq./1,000 kcal | No seafood or plant proteins | 5 |
| Fatty acids *(MUFA+PUFA)/SFA* | ≥2.5 | ≤1.2 | 10 |
| Refined grains | ≤1.8 oz eq./1,000 kcal | ≥4.3 oz eq./1,000 kcal | 10 |
| Sodium | ≤1.1 g/1,000 kcal | ≥2.0 g/1,000 kcal | 10 |
| Added sugars | ≤6.5% of energy | ≥26% of energy | 10 |
| Saturated fat | ≤8% of energy | ≥16% of energy | 10 |

## Other Indices

The DII, MIND, AHEI, DASH, PHDI, MEDI, and ACS2020 scores follow the formulas described in [validation.md](../validation.md). Each function in `compute/` mirrors the reference implementation from the R `dietaryindex` package.
