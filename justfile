export RUSTFLAGS := "-Z macro-backtrace --cfg tokio_unstable"
dev_compose_file := "./dev.compose.yml"

dev-compose-down:
    docker compose -f {{ dev_compose_file }} down

dev-compose-up:
    docker compose -f {{ dev_compose_file }} up -d

dev-compose-restart:
    just dev-compose-down
    just dev-compose-up

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
    cargo run --bin {{ crate }}

check crate:
    just udeps && \
    just audit && \
    just fmt && \
    just lint && \
    just test && \
    just run {{ crate }}

watch-rs crate:
    watchexec \
        -rqc reset \
        -e rs,toml \
        "just check {{ crate }}"

sqlx-prepare:
    cargo sqlx prepare

watch-sql:
    watchexec \
        -rqc reset \
        -e sql \
        "just sqlx-prepare"
