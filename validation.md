# 🧪 Validation & Scoring Standards

This document provides comprehensive, science-based validation details for each diet-quality index implemented in the **Dietary Index Web Calculator**. It includes calculation methods, reference sources, expected ranges, and validation strategies.

---

## 1. Dietary Inflammatory Index (DII)
**Purpose:** Quantify the inflammatory potential of a diet.
**Range:** −8.87 (most anti-inflammatory) to +7.98 (most pro-inflammatory).
**Units:** Dimensionless score normalized to world-standard intake.

### 1.1 Calculation
1. **Literature-derived parameters** for 45 food parameters (e.g., nutrients, flavonoids).
2. **Standardize intake**: \(Z_i = rac{(X_i - \mu_i)}{\sigma_i}\), where \(X_i\) is daily intake, \(\mu_i\)/\(\sigma_i\) are global means/SDs.
3. **Percentile conversion**: Convert \(Z_i\) to percentile (0–1).
4. **Centered percentile**: \(CP_i = 2 	imes percentile_i - 1\).
5. **Weighted sum**: \(DII = \sum_{i=1}^{N} (CP_i 	imes W_i)\), where \(W_i\) is the inflammatory effect score from Shivappa et al. (2014).

### 1.2 References
- Hébert JR, Shivappa N, et al. *Designing and developing a literature-derived, population-based Dietary Inflammatory Index.* Public Health Nutr. 2014.
- Hébert JR, et al. *Perspective: Dietary Inflammatory Index—Lessons Learned, Improvements Made, and Future Directions.* Adv Nutr. 2019.
- Wikipedia: Dietary Inflammatory Index citeturn0search1

---

## 2. MIND Diet Score
**Purpose:** Assess adherence to the Mediterranean-DASH Intervention for Neurodegenerative Delay diet, targeting brain health.
**Range:** 0 (lowest adherence) to 15 (highest adherence).
**Units:** Sum of component scores.

### 2.1 Calculation
- **15 dietary components** (10 brain-healthy, 5 unhealthy).
- **Per-component scoring**: 0 (lowest), 0.5 (medium), 1 (highest) based on weekly consumption thresholds.
- **Total score**: Sum across components, e.g.,
  ```text
  Score = Σ_{healthy} (servings ≥ threshold ? 1 : servings in medium range ? 0.5 : 0)
        + Σ_{unhealthy} (servings ≤ threshold ? 1 : servings in medium range ? 0.5 : 0)
  ```

### 2.2 References
- Morris MC, Tangney CC, et al. *MIND diet associated with reduced incidence of Alzheimer’s Disease.* Alzheimers Dement. 2015.
- Rush University Memory and Aging Project documentation citeturn1search0

---

## 3. Healthy Eating Index 2015 (HEI‑2015)
**Purpose:** Measure conformance to 2015–2020 U.S. Dietary Guidelines.
**Range:** 0 to 100.
**Units:** Sum of component scores (density per 1,000 kcal).

### 3.1 Components & Scoring
- **13 components**: 9 adequacy (e.g. total fruit, whole grains) + 4 moderation (e.g. added sugars, saturated fat).
- **Density basis**: e.g., cups per 1,000 kcal or ratio of fatty acids.
- **Scoring**:
  \[
    component\ score =
    egin{cases}
      max\ score & intake ≥ standard_{max} \
      (intake / standard_{max}) × max\ score & standard_{min} < intake < standard_{max} \
      0 & intake ≤ standard_{min}
    \end{cases}
  \]

- **Total HEI‑2015** = Σ component scores.

### 3.2 References
- Krebs-Smith SM, Pannucci TE, et al. *Update of the Healthy Eating Index: HEI‑2015.* J Acad Nutr Diet. 2018.
- NIH NCI HEI Scoring Algorithm: EGRP/DCCPS/NCI/NIH citeturn2search0

---
## 5. Alternative Healthy Eating Index (AHEI)
**Purpose:** Evaluate diet quality for chronic disease prevention.
**Range:** 0 to 110.
**Units:** Sum of component scores (11 components × 10 points).

### 5.1 Calculation
- **11 components** including vegetables, fruit, whole grains, nuts/legumes,
  omega‑3 fats, polyunsaturated fat, sugar‑sweetened beverages,
  red/processed meat, trans fat, sodium, and alcohol.
- **Linear scoring** from 0–10 with gender‑specific cut points for whole grains
  and alcohol. Sodium is scored on energy-adjusted deciles.

### 5.2 References
- Chiuve SE, Fung TT, et al. *Development of the Alternative Healthy Eating
  Index‑2010.* J Nutr. 2012.
- Harvard T.H. Chan School of Public Health: AHEI scoring method.

## 6. AHEI (Serving-based Variant, AHEIP)
**Purpose:** Serving-sized adaptation used in epidemiologic studies.
**Range:** 0 to 90.
**Units:** Sum of component scores (9 components × 10 points).

### 6.1 Calculation
- Scores servings of vegetables, whole fruit, white‑to‑red meat ratio, fiber,
  trans fat, polyunsaturated‑to‑saturated fat ratio, calcium, folate, and iron.
- Healthy components score linearly upward; unhealthy components reverse score.

### 6.2 References
- `dietaryindex` R package documentation.

## 7. Alternate Mediterranean Diet Score (aMED)
**Purpose:** U.S. adaptation of the Mediterranean diet pattern.
**Range:** 0 to 9.
**Units:** Sum of binary component scores.

### 7.1 Calculation
- Components compared to cohort medians with 1 point for favorable intake.
- Alcohol earns 1 point for 10–25 g/day.

### 7.2 References
- Fung TT, McCullough ML, et al. *Mediterranean diet and mortality in the
  NIH‑AARP Diet and Health Study.* Arch Intern Med. 2009.

## 8. MED Index in Serving Sizes (MEDI)
**Purpose:** PREDIMED-style Mediterranean index using serving thresholds.
**Range:** 0 to 11.
**Units:** Sum of binary component scores.

### 8.1 Calculation
- Each component earns 1 point when the PREDIMED threshold is met.

### 8.2 References
- Estruch R, et al. *Primary Prevention of Cardiovascular Disease with a
  Mediterranean Diet.* N Engl J Med. 2013.

## 9. Healthy Eating Index 2020
**Purpose:** Measure conformance to the 2020–2025 U.S. Dietary Guidelines.
**Range:** 0 to 100.
**Variants:** HEI‑2020 for adults and HEI‑Toddlers‑2020 for children aged 1–2 years.

### 9.1 Calculation
- Density-based approach similar to HEI‑2015 with updated cut points.

### 9.2 References
- USDA Center for Nutrition Policy and Promotion HEI‑2020 materials.

## 10. Planetary Health Diet Index (PHDI)
**Purpose:** Quantify adherence to the EAT‑Lancet Planetary Health Diet.
**Range:** 0 to 140.
**Units:** Sum of component scores.

### 10.1 Calculation
- Components scaled 0–10 (legumes and soy max 5) with gender-specific cut
  points for whole grains.

### 10.2 References
- Cacau LT, et al. *Development and validation of the Planetary Health Diet
  Index.* Nutrients. 2023.
- EAT‑Lancet Commission documentation.

## 11. American Cancer Society 2020 Diet Score
**Purpose:** Evaluate compliance with 2020 ACS guidelines for cancer prevention.
**Range:** 0 to 12.
**Variants:** `ACS2020_V1` uses percent of calories from highly processed foods
and refined grains; `ACS2020_V2` uses servings per 1,000 kcal.

### 11.1 Calculation
- Components are ranked in gender-specific quartiles. Healthy foods earn up to
  0.75 or 3 points; unhealthy foods are reverse scored. Sugar-sweetened
  beverages use fixed cut points.

### 11.2 References
- Kushi LH, et al. *American Cancer Society guideline for diet and physical
  activity for cancer prevention.* CA Cancer J Clin. 2020.

_Last updated: 2025-06-29_

## 🚧 Implementation Note (2025‑06‑28)

The web UI now tries to compute **all** four diet indices automatically when the
necessary columns are present. Only the DII calculation has been validated with
real data so far. MIND, HEI‑2015, and DASH still need dedicated input datasets
and benchmark results. Once those resources are available, revisit this logic
and confirm the implementations.
**Purpose:** Quantify adherence to the Dietary Approaches to Stop Hypertension diet.
**Common Range:** 8 (lowest adherence) to 40 (highest adherence).
**Units:** Sum of component quintile-based scores.

### 4.1 Calculation (Fung et al. method)
1. **8 components**: Fruits, vegetables (excl. potatoes), whole grains, low-fat dairy, nuts/legumes, sodium, red/processed meats, sweetened beverages.
2. **Cohort-specific quintiles**: Assign 1–5 points for each adequacy component (highest quintile = 5) and reverse for moderation components (lowest quintile = 5).
3. **Total** = Σ component scores (range 8–40).

### 4.2 References
- Fung TT, Chiuve SE, et al. *Adherence to a DASH-style diet and risk of hypertension in women.* JAMA. 2008.
- Wikipedia: DASH diet citeturn4search0

---

## 📋 Validation Workflow

1. **Unit tests**: Compare outputs to gold-standard datasets (`tests/data/`).
2. **Statistical checks**: Ensure score distributions match published summary statistics.
3. **End-to-end**: CSV → API → JSON & CSV outputs validated against expected ranges.
4. **Visualization**: Plotly histograms & boxplots match known population distributions.

_Last updated: 2025-06-27_
