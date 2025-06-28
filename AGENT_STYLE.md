# AGENT_STYLE.md

## Purpose

This living style guide captures the visual and experiential standards for the project. All explicit or implied user requests about look, feel, and UI/UX are boiled down here to create a "billion dollar company" user experience. Update this doc whenever major style feedback is received.

---

## Central Styling OKRs

**1. White-Glove, Billion Dollar Corporation Look**
- Everything must feel ultra-premium, like the best of Google, Apple, OpenAI, or a top S&P 5 company.
- Design must create a sense of calm, clarity, and polish—never chaotic or cluttered.
- Every detail is intentional. Every element is spaced, aligned, and presented to perfection (OCD-level, AI-grade attention).

**2. University of Florida-Inspired Palette**
- **Background:** Pure white (#FFFFFF).
- **Primary Text:** Deep UF Blue (recommended: `#002D72` for main text).
- **Highlight Text/Elements:** Use standard UF Blue (`#1976D2` or similar) and UF Orange (`#FA4616`) for accentuating links, buttons, and highlights.
- **All text must have strong contrast and appear bold, clean, and modern.**

**3. Header & Logo**
- The header image (UFLogo.png) is **centered, perfectly sized** for all screens—fills width on phones, scales proportionally for tablets, laptops, and desktops.
- **Header logo must be clickable**, linking to [https://hobi.med.ufl.edu/](https://hobi.med.ufl.edu/).
- Never alter placement, spacing, or aspect ratio of the header image.

**4. Typography**
- Use a modern, sans-serif font (e.g., Inter, Helvetica Neue, Roboto).
- Headings: bold, slightly larger, always deep UF Blue.
- Body: clean, deep blue, medium/large for legibility.
- Small text: Use for tooltips, annotations, with lighter blue or muted tone.
- Never crowd text; plenty of whitespace.

**5. UI Components**
- Backgrounds: pure white.
- Cards, buttons, and containers: softly rounded corners (border-radius: 1.5rem+), gentle drop shadows, plush padding.
- Buttons: minimal, bold, colored with UF Blue or Orange accents, soft hover transitions.
- Tooltips and modals: simple, white, rounded, shadowed, with small, sharp text.
- Forms: clean, clear, labeled, and aligned with rest of the UI.

**6. Layout & Responsiveness**
- Grid-based, always centered, balanced.
- Sizing adapts perfectly from mobile (edge-to-edge logo/image) to widescreen.
- Navigation, content, and footer sections are always evenly spaced, never cramped.

**7. Visual Touches**
- No harsh lines or colors; all transitions and effects should be soft and subtle.
- Animation: Minimal—use only for subtle motion (e.g., header fade, button hover), preferably via CSS transitions or a modern JS lib (Framer Motion, etc.).
- All visuals are precise, symmetrical, and pleasing to the eye.

**8. Overall Feel**
- If it doesn’t look and feel like a billion-dollar product, it isn’t done.
- “Perfectionist AI” mindset—nothing overlooked.

---

## Example Snippet

```html
<!-- Header logo: always centered, responsive, clickable -->
<a href="https://hobi.med.ufl.edu/" style="display:block; text-align:center; margin-top:2rem;">
  <img src="assets/UFLogo.png" alt="UF Logo" style="max-width:95vw; width:100%; height:auto; border-radius:2rem; box-shadow:0 8px 32px rgba(0,45,114,0.08);"/>
</a>
```
