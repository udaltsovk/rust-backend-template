FROM lukemathwalker/cargo-chef:latest-rust-1.92-trixie AS chef
COPY rust-toolchain.toml rust-toolchain.toml
WORKDIR /usr/src

ARG RUSTFLAGS="--cfg tokio_unstable -Z threads=8"
ENV RUSTFLAGS=${RUSTFLAGS}

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ENV SQLX_OFFLINE=true

COPY --from=planner /usr/src/recipe.json recipe.json
RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=target,target=/usr/src/target \
	cargo chef cook --release --locked --recipe-path recipe.json --bin template_example-monolyth

COPY . .

RUN --mount=type=cache,id=cargo-registry,target=/usr/local/cargo/registry \
    --mount=type=cache,id=cargo-git,target=/usr/local/cargo/git \
    --mount=type=cache,id=target,target=/usr/src/target \
    cargo build --release --locked --bin template_example-monolyth \
    && cp /usr/src/target/release/template_example-monolyth /usr/src/app


FROM gcr.io/distroless/cc-debian13:nonroot

LABEL org.opencontainers.image.source=https://github.com/udaltsovk/rust-backend-template
LABEL org.opencontainers.image.title=rust_template_example
LABEL org.opencontainers.image.description="Rust Template Backend App Example"

WORKDIR /app

COPY --from=builder /usr/src/app app

CMD ["/app/app"]
