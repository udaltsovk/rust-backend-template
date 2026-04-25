set dotenv-load
set dotenv-required

database_url := "DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DATABASE"
dev_compose_file := "./dev.compose.yml"
default_app_name := "template_example"

dev-compose-down:
    docker compose -f {{ dev_compose_file }} down

dev-compose-up:
    docker compose -f {{ dev_compose_file }} up -d

dev-compose-restart: dev-compose-down dev-compose-up

udeps *args="--all":
    cargo udeps {{ args }}

audit *args="-n":
    cargo audit {{ args }}

fmt *args="--all":
    cargo fmt {{ args }}

lint *args="--workspace --all-targets -- -D warnings":
    cargo clippy {{ args }}

fix *args="--workspace --all-targets":
    just lint --fix {{ args }}

test *args="--workspace":
    cargo test {{ args }}

build crate=(default_app_name + "-monolyth") *args:
    cargo build --bin {{ crate }} {{ args }}

style: fmt lint

check: udeps audit style test

run crate=(default_app_name + "-monolyth") *args:
    cargo run --bin {{ crate }} {{ args }}

watch-rs crate=(default_app_name + "-monolyth"):
    watchexec \
        -rqc reset \
        -e rs,toml,lock \
        "just style run {{ crate }}"

sqlx-reset crate=default_app_name db="postgres" *args:
    migration_dirs="$$(find ./apps/{{ crate }}/src/features -type d -path '*/infrastructure/persistence/{{ db }}/migration' | sort)" && \
    count="$$(printf '%s\n' "$$migration_dirs" | sed '/^$/d' | wc -l | tr -d ' ')" && \
    [ "$$count" -eq 1 ] || { echo "expected exactly one {{ db }} migration directory, found $$count" >&2; exit 1; } && \
    {{ database_url }} cargo sqlx database reset --source "$$migration_dirs" {{ args }}

sqlx-prepare crate=default_app_name db="postgres" *args:
    cd ./apps/{{ crate }} && \
    {{ database_url }} cargo sqlx prepare {{ args }}

watch-sql crate=default_app_name:
    watchexec \
        -rqc reset \
        -e sql \
        "just sqlx-prepare {{ crate }}"
