
FROM lukemathwalker/cargo-chef:latest-rust-1.91-trixie AS chef
WORKDIR /usr/src/app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder

ENV OPENSSL_STATIC=1

COPY --from=planner /usr/src/app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json --bin ad_platform-monolyth

COPY . .
COPY .git .git
RUN SQLX_OFFLINE=true cargo build --release --bin ad_platform-monolyth


FROM debian:trixie-slim

LABEL org.opencontainers.image.source=https://github.com/udaltsovk/rust-backend-template
LABEL org.opencontainers.image.title=ad_platform
LABEL org.opencontainers.image.description="Ad Platform"

RUN apt-get update \
	&& apt-get install -y --no-install-recommends ca-certificates dumb-init curl \
	&& rm -rf /var/lib/apt/lists/*

WORKDIR /app

RUN useradd -MN app
USER app

COPY --from=builder /usr/src/app/target/release/ad_platform-monolyth app

CMD ["/app/app"]
