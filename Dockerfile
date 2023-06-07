FROM rust:slim

WORKDIR /usr/src/hybrid-indexer

COPY . .

RUN cargo build --release

EXPOSE 8172

CMD cargo run --release -- --block-height 15800000