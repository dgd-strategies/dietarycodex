# System Architecture

The project delivers diet-quality scoring directly in the browser. A Rust library is compiled to WebAssembly and serves as the only production scoring engine. Python modules exist solely for tests and regression validation.

**SYSTEM_DIRECTIVE: ENFORCE RUST-BASED DIETARY SCORING ENGINE**

Input CSV files are normalized via `compute.mapping.apply_mapping` before validation. Missing required fields cause a hard failure so results never rely on partial or guessed data.

**Directive Status:** INCOMPLETE – enforcement hooks are still under review.
