# Serving Size Lookup

This document lists clean formulas for converting raw dietary variables to standardized serving sizes. Each dataset uses slightly different variable names, so the table below shows how we derive servings per 1,000 kcal.

## NHANES

| Component | Formula | Unit |
|-----------|---------|------|
| Total fruit | `DR1T_F_TOTAL / (DR1TKCAL / 1000)` | cup eq./day |
| Whole fruit | `(DR1T_F_CITMLB + DR1T_F_OTHER) / (DR1TKCAL / 1000)` | cup eq./day |
| Total vegetables | `(DR1T_V_TOTAL + DR1T_V_LEGUMES) / (DR1TKCAL / 1000)` | cup eq./day |
| Whole grains | `DR1T_G_WHOLE / (DR1TKCAL / 1000)` | oz eq./day |
| Seafood & plant protein | `(DR1T_PF_SEAFD_HI + DR1T_PF_SEAFD_LOW + DR1T_PF_NUTSDS + DR1T_PF_SOY + DR1T_PF_LEGUMES) / (DR1TKCAL / 1000)` | oz eq./day |

## ASA24

| Component | Formula | Unit |
|-----------|---------|------|
| Total fruit | `F_TOTAL / (KCAL / 1000)` | cup eq./day |
| Whole fruit | `(F_CITMLB + F_OTHER) / (KCAL / 1000)` | cup eq./day |
| Total vegetables | `(V_TOTAL + V_LEGUMES) / (KCAL / 1000)` | cup eq./day |
| Whole grains | `G_WHOLE / (KCAL / 1000)` | oz eq./day |
| Seafood & plant protein | `(PF_SEAFD_HI + PF_SEAFD_LOW + PF_NUTSDS + PF_SOY + PF_LEGUMES) / (KCAL / 1000)` | oz eq./day |

## DHQ3

| Component | Formula | Unit |
|-----------|---------|------|
| Total fruit | `FRUIT_TOTAL / (ENERGY / 1000)` | cup eq./day |
| Whole fruit | `FRUIT_WHOLE / (ENERGY / 1000)` | cup eq./day |
| Total vegetables | `(VEG_TOTAL + VEG_LEGUMES) / (ENERGY / 1000)` | cup eq./day |
| Whole grains | `GRAIN_WHOLE / (ENERGY / 1000)` | oz eq./day |
| Seafood & plant protein | `(PROT_SEA_HI + PROT_SEA_LOW + PROT_NUTS + PROT_SOY + PROT_LEGUMES) / (ENERGY / 1000)` | oz eq./day |

## BLOCK FFQ

| Component | Formula | Unit |
|-----------|---------|------|
| Total fruit | `FRUIT_TOTAL / (KCAL / 1000)` | cup eq./day |
| Whole fruit | `FRUIT_WHOLE / (KCAL / 1000)` | cup eq./day |
| Total vegetables | `(VEG_TOTAL + VEG_LEGUMES) / (KCAL / 1000)` | cup eq./day |
| Whole grains | `GRAIN_WHOLE / (KCAL / 1000)` | oz eq./day |
| Seafood & plant protein | `(SEA_HI + SEA_LOW + NUTS + SOY + LEGUMES) / (KCAL / 1000)` | oz eq./day |
