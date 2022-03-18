FROM rustlang/rust:nightly as build


RUN USER=root cargo new --bin rust-microservices
WORKDIR /rust-microservices
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build

RUN rm src/*.rs
COPY ./src ./src
RUN rm ./target/debug/deps/*
RUN cargo build
RUN rm ./target/debug/deps/*

FROM debian:stable-slim

RUN apt-get update -qq && \
    apt-get install -y libpq-dev  && \
    rm -rf /var/lib/apt/lists/*

COPY --from=build /rust-microservices/target/debug/back-end .

CMD ["./back-end"]

EXPOSE 80