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
    cargo clippy --workspace --all-targets -- -D warnings

test:
    cargo test --all

build crate:
    cargo build --bin {{ crate }}

check crate:
    just udeps && \
    just audit && \
    just fmt && \
    just lint && \
    just test && \
    just build {{ crate }}

run crate:
    cargo run --bin {{ crate }}

watch-rs crate:
    watchexec \
        -rqc reset \
        -e rs,toml,lock \
        "just check {{ crate }} && just run {{ crate }}"

sqlx-prepare crate db="postgres":
    cd ./apps/{{ crate }}/infrastructure/persistence/{{ db }} && \
    cargo sqlx prepare

watch-sql:
    watchexec \
        -rqc reset \
        -e sql \
        "just sqlx-prepare"
