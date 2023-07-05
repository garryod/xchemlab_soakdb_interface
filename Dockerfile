FROM docker.io/library/rust:1.70.0-bullseye AS build

WORKDIR /app

# Build dependencies
COPY Cargo.toml Cargo.lock ./
COPY chimp_chomp/Cargo.toml chimp_chomp/Cargo.toml
COPY chimp_protocol/Cargo.toml chimp_protocol/Cargo.toml
COPY graphql_endpoints/Cargo.toml graphql_endpoints/Cargo.toml
COPY graphql_event_broker/Cargo.toml graphql_event_broker/Cargo.toml
COPY opa_client/Cargo.toml opa_client/Cargo.toml
COPY pin_packing/Cargo.toml pin_packing/Cargo.toml
COPY soakdb_io/Cargo.toml soakdb_io/Cargo.toml
COPY soakdb_sync/Cargo.toml soakdb_sync/Cargo.toml
RUN mkdir chimp_chomp/src \
    && touch chimp_chomp/src/lib.rs \
    && mkdir chimp_protocol/src \
    && touch chimp_protocol/src/lib.rs \
    && mkdir graphql_endpoints/src \
    && touch graphql_endpoints/src/lib.rs \
    && mkdir graphql_event_broker/src \
    && touch graphql_event_broker/src/lib.rs \
    && mkdir opa_client/src \
    && touch opa_client/src/lib.rs \
    && mkdir pin_packing/src/ \
    && echo "fn main() {}" > pin_packing/src/main.rs \
    && mkdir soakdb_io/src \
    && touch soakdb_io/src/lib.rs \
    && mkdir soakdb_sync/src/ \
    && echo "fn main() {}" > soakdb_sync/src/main.rs \
    && cargo build --release

# Build workspace crates
COPY . /app
RUN touch chimp_chomp/src/lib.rs \
    && touch chimp_protocol/src/lib.rs \
    && touch graphql_endpoints/src/lib.rs \
    && touch graphql_event_broker/src/lib.rs \
    && touch opa_client/src/lib.rs \
    && touch pin_packing/src/main.rs \
    && touch soakdb_io/src/lib.rs \
    && touch soakdb_sync/src/main.rs \
    && cargo build --release

FROM gcr.io/distroless/cc
ARG SERVICE

COPY --from=build /app/target/release/${SERVICE} /service

ENTRYPOINT ["./service"]
