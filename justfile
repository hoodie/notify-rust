default:
    just --list

coverage:
    # cargo llvm-cov test --lib --no-default-features --features async_runtime
    # cargo llvm-cov test --lib --no-default-features --features tokio_runtime
    # cargo llvm-cov test --lib
    cargo llvm-cov --html test --lib
    open target/llvm-cov/html/index.html

clippy:
    cargo --quiet linux-clippy --workspace --quiet
    cargo --quiet mac-clippy --workspace --quiet
    cargo --quiet mac-clippy --workspace --quiet --features preview-macos-un
    cargo --quiet win-clippy --workspace --quiet

check:
    cargo --quiet linux-check --workspace --quiet
    cargo --quiet mac-check --workspace --quiet
    cargo --quiet mac-check --workspace --quiet --features preview-macos-un
    cargo --quiet win-check --workspace --quiet
