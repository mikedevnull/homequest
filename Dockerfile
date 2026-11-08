FROM rust:1-bookworm AS builder

WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
COPY migrations ./migrations
COPY .sqlx ./.sqlx
COPY default_config.toml ./default_config.toml
ENV SQLX_OFFLINE=true
RUN cargo build --release --bin homequest

FROM debian:bookworm-slim

RUN apt-get update \
    && apt-get install -y --no-install-recommends ca-certificates libssl3 \
    && rm -rf /var/lib/apt/lists/* \
    && useradd --system --uid 10001 --create-home homequest \
    && mkdir /app /data \
    && chown homequest:homequest /app /data
WORKDIR /app
COPY --from=builder --chown=homequest:homequest /app/target/release/homequest /usr/local/bin/homequest
COPY --chown=homequest:homequest templates ./templates

ENV HOMEQUEST_HOST=0.0.0.0
ENV HOMEQUEST_DATABASE_URL=sqlite:///data/homequest.db?mode=rwc
EXPOSE 8080
VOLUME ["/data"]
USER homequest
ENTRYPOINT ["homequest"]