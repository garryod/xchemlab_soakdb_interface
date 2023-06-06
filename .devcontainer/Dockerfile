FROM docker.io/library/rust:1.70.0-bullseye

RUN rustup component add rustfmt clippy

RUN apt-get update \
    && apt-get install --yes --no-install-recommends \
    sqlite3 pre-commit \
    && rm -rf /var/lib/apt/lists/*
