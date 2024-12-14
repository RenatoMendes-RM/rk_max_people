FROM rust:latest AS build

WORKDIR /usr/src/rk_max_people

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs && \
    cargo build --release && \
    rm -f target/release/deps/rk_max_people*

COPY . .

RUN cargo build --release

# Etapa de ejecución
FROM debian:bullseye-slim

WORKDIR /usr/src/rk_max_people

COPY --from=build /usr/src/rk_max_people/target/release/rk_max_people .

EXPOSE 8072

CMD ["./rk_max_people"]