# Feature combinations from .github/workflows/build-platforms.yml (linux job)
# Features: d | d,images | z | z,images | z,d

# Run all Linux CI checks for every feature combination
default:
    @just --list

# --- default features ---

check-default:
    cargo check

clippy:
    cargo clippy

# --- per-feature helpers (internal) ---

[private]
check-features features:
    cargo check --no-default-features --features {{features}}

[private]
test-lib features:
    cargo test --lib --no-default-features --features {{features}}

[private]
test-doc features:
    cargo test --doc --no-default-features --features {{features}}

[private]
test-features features: (test-lib features) (test-doc features)

# --- individual feature-set targets ---

check-d:           (check-features "d")
check-d-images:    (check-features "d,images")
check-z:           (check-features "z")
check-z-images:    (check-features "z,images")
check-z-d:         (check-features "z,d")

test-d:            (test-features "d")
test-d-images:     (test-features "d,images")
test-z:            (test-features "z")
test-z-images:     (test-features "z,images")
test-z-d:          (test-features "z,d")

# --- aggregate targets ---

check-all: check-default check-d check-d-images check-z check-z-images check-z-d

test-all: test-d test-d-images test-z test-z-images test-z-d

# Run the full Linux CI matrix locally (check + test + clippy)
ci: check-all test-all clippy
