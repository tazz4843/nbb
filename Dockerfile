FROM rustlang/rust:nightly-slim as build

WORKDIR /nbb

COPY . .

ENV RUSTFLAGS="--emit=asm"

RUN cargo build --release

# our final base
FROM debian:bullseye-slim

WORKDIR /

COPY --from=build /nbb/target/release/nbb .

EXPOSE 8080/tcp
VOLUME "/blog"

CMD ["./nbb"]