export RUSTFLAGS := "-Z macro-backtrace --cfg tokio_unstable"
database_url := "DATABASE_URL=postgres://$POSTGRES_USER:$POSTGRES_PASSWORD@$POSTGRES_HOST:$POSTGRES_PORT/$POSTGRES_DATABASE"
dev_compose_file := "./dev.compose.yml"
default_app_name := "template_example"

dev-compose-down:
    docker compose -f {{ dev_compose_file }} down

dev-compose-up:
    docker compose -f {{ dev_compose_file }} up -d

dev-compose-restart:
    just dev-compose-down dev-compose-up

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

coverage *args="--skip-clean --workspace --all-targets -o Xml -o Html":
    cargo tarpaulin {{ args }}
    pycobertura show cobertura.xml

build crate=(default_app_name + "-monolyth") *args:
    cargo build --bin {{ crate }} {{ args }}

style:
    just fmt
    just lint

check:
    just udeps
    just audit
    just style
    just coverage

run crate=(default_app_name + "-monolyth") *args:
    cargo run --bin {{ crate }} {{ args }}

watch-rs crate=(default_app_name + "-monolyth"):
    watchexec \
        -rqc reset \
        -e rs,toml,lock \
        "just style run {{ crate }}"

sqlx-reset crate=(default_app_name + "-monolyth") db="postgres" *args:
    {{ database_url }} cargo sqlx database reset --source ./apps/{{ crate }}/infrastructure/persistence/{{ db }}/migrations {{ args }}

sqlx-prepare crate=(default_app_name + "-monolyth") db="postgres" *args:
    cd ./apps/{{ crate }}/infrastructure/persistence/{{ db }} && \
    {{ database_url }} cargo sqlx prepare {{ args }}

watch-sql crate=(default_app_name + "-monolyth"):
    watchexec \
        -rqc reset \
        -e sql \
        "just sqlx-prepare {{ crate }}"
