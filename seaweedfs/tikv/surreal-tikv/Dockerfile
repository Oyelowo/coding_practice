FROM rust:slim-buster as builder

RUN cargo new --bin surreal-tikv
WORKDIR /surreal-tikv
COPY ./Cargo.toml ./Cargo.toml
COPY ./src ./src
RUN cargo build


FROM debian:buster-slim
COPY --from=builder /surreal-tikv/target/debug/surreal-tikv ./surreal-tikv
CMD ["./surreal-tikv"]