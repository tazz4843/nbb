FROM rustlang/rust:nightly-slim as build

WORKDIR /nbb

COPY . .

ENV RUSTFLAGS="--emit=asm"

RUN cargo build --release

# our final base
FROM debian:bullseye-slim

# copy the build artifact from the build stage
COPY --from=build /nbb/target/release/nbb .

EXPOSE 8080/tcp
VOLUME "/config"
VOLUME "/blog"

# set the startup command to run your binary
CMD ["./nbb", "/config/config.yml"]
