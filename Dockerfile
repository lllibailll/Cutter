FROM rustlang/rust:nightly AS builder

RUN rustup target add x86_64-unknown-linux-gnu --toolchain=nightly

RUN USER=root cargo new --bin cutter
WORKDIR /cutter

COPY ./Cargo.lock ./Cargo.lock
COPY ./Cargo.toml ./Cargo.toml

COPY ./src ./src

RUN cargo build --release --target x86_64-unknown-linux-gnu

FROM ubuntu:18.04
COPY --from=builder /cutter/target/x86_64-unknown-linux-gnu/release/cutter /cutter/

WORKDIR /cutter/

ENV RUST_LOG=info
ENV RUST_BACKTRACE=full

RUN apt-get update && apt-get install -y libmariadb-dev

ENV ROCKET_DATABASES='{Cutter={url="mysql://user:password@host/DB"}}'

CMD ["./cutter"]
