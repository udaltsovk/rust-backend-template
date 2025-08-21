export RUSTFLAGS := "-Z macro-backtrace --cfg tokio_unstable"

udeps:
    cargo udeps --all

audit:
    cargo audit -n

fmt:
    cargo fmt --all

lint:
    cargo clippy --all -- -D warnings

test:
    cargo test --all

run crate:
    cargo run --bin {{crate}}

watch-rs crate:
    watchexec \
        -rqc reset \
        -e rs,toml \
        "just udeps && \
         just audit && \
         just fmt && \
         just lint && \
         just test && \
         just run {{crate}}"

migrate:
    cargo sqlx prepare --workspace

watch-migrations:
    watchexec \
        -rqc reset \
        -e sql \
        "just migrate"

watch-all crate:
    just --evaluate watch-rs {{crate}} &
    just --evaluate watch-migrations &
    wait
