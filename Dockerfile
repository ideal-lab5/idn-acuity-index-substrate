FROM rust:1.68.2

WORKDIR /usr/src/hybrid-indexer

COPY . .

RUN rustup toolchain install nightly
RUN cargo +nightly build --release

EXPOSE 8080

CMD cargo +nightly run --release -- --url wss://rpc.polkadot.io:443 --block-height 13800000