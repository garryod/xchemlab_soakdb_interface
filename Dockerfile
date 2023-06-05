FROM docker.io/library/rust:1.70.0-bullseye AS build

WORKDIR /app

# Build dependencies
COPY Cargo.toml Cargo.lock .
COPY soakdb/Cargo.toml soakdb/Cargo.toml
COPY auth/Cargo.toml auth/Cargo.toml
COPY backend/Cargo.toml backend/Cargo.toml
RUN mkdir soakdb/src \
    && touch soakdb/src/lib.rs \
    && mkdir auth/src \
    && touch auth/src/lib.rs \
    && mkdir backend/src/ \
    && echo "fn main() {}" > backend/src/main.rs \
    && cargo build --release

# Build workspace crates
COPY . /app
RUN touch soakdb/src/lib.rs \
    && touch auth/src/lib.rs \
    && touch backend/src/main.rs \
    && cargo build --release

FROM gcr.io/distroless/cc

COPY --from=build /app/target/release/backend /

ENTRYPOINT ["./backend"]
