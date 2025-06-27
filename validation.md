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

## 4. DASH Diet Score
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
