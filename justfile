export RUSTFLAGS := "-Z macro-backtrace --cfg tokio_unstable"
dev_compose_file := "./dev.compose.yml"
default_app_name := "template_example"

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

build crate="{{ default_app_name }}-monolyth":
    cargo build --bin {{ crate }}

check:
    just udeps && \
    just audit && \
    just fmt && \
    just lint && \
    just test

run crate="{{ default_app_name }}-monolyth":
    cargo run --bin {{ crate }}

watch-rs crate="{{ default_app_name }}-monolyth":
    watchexec \
        -rqc reset \
        -e rs,toml,lock \
        "just check && just run {{ crate }}"

sqlx-reset crate="{{ default_app_name }}-monolyth" db="postgres":
    cargo sqlx database reset --source ./apps/{{ crate }}/infrastructure/persistence/{{ db }}/migrations

sqlx-prepare crate="{{ default_app_name }}-monolyth" db="postgres":
    cd ./apps/{{ crate }}/infrastructure/persistence/{{ db }} && \
    cargo sqlx prepare

watch-sql crate="{{ default_app_name }}-monolyth":
    watchexec \
        -rqc reset \
        -e sql \
        "just sqlx-prepare {{ crate }}"
