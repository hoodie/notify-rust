default:
    just --list

check:
    cargo check --workspace
    cargo check --workspace --no-default-features --features z-with-tokio
    cargo check --workspace --no-default-features --features async
    cargo check --workspace --no-default-features --features async,tokio
    cargo check --workspace --no-default-features --features macos_legacy

coverage:
    # cargo llvm-cov test --lib --no-default-features --features async_runtime
    # cargo llvm-cov test --lib --no-default-features --features tokio_runtime
    # cargo llvm-cov test --lib
    cargo llvm-cov --html test --lib
    open target/llvm-cov/html/index.html

clippy:
    cargo --quiet clippy --workspace --quiet
    cargo --quiet clippy --workspace --quiet --lib --tests --no-default-features --features z-with-tokio
    cargo --quiet clippy --workspace --quiet --lib --tests --no-default-features --features async
    cargo --quiet clippy --workspace --quiet --lib --tests --no-default-features --features async,tokio
    cargo --quiet clippy --workspace --quiet --lib --tests --no-default-features --features macos_legacy

test $RUST_LOG="trace" $STRESS_COUNT="3":
    cargo nextest run --profile ci --workspace --all-targets --stress-count $STRESS_COUNT
    cargo nextest run --profile ci --workspace --lib --no-default-features --features z-with-tokio --stress-count $STRESS_COUNT
    cargo nextest run --profile ci --workspace --lib --no-default-features --features async --stress-count $STRESS_COUNT
    cargo nextest run --profile ci --workspace --lib --no-default-features --features async,tokio --stress-count $STRESS_COUNT
    cargo nextest run --profile ci --workspace --lib --no-default-features --features macos_legacy --stress-count $STRESS_COUNT

doc_test:
    cargo test --workspace --doc --no-default-features --features z-with-tokio
    cargo test --workspace --doc --no-default-features --features async
    cargo test --workspace --doc --no-default-features --features async,tokio
    cargo test --workspace --doc --no-default-features --features macos_legacy
    cargo doc --no-deps

install-deps:
    @cargo install cargo-nextest
    @cargo install cargo-semver-checks

semver-checks:
    cargo semver-checks --only-explicit-features --features z-with-tokio
    cargo semver-checks --only-explicit-features --features async
    cargo semver-checks --only-explicit-features --features async,tokio
    cargo semver-checks --only-explicit-features --features macos_legacy

build-examples:
    cargo build --examples
    cargo build --examples --no-default-features --features z-with-tokio
    cargo build --examples --no-default-features --features async
    cargo build --examples --no-default-features --features async,tokio
    cargo build --examples --no-default-features --features macos_legacy

ci: install-deps clippy test build-examples doc_test semver-checks
