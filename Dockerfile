ARG BASE_IMAGE=ekidd/rust-musl-builder

# Our first FROM statement declares the build environment.
FROM ${BASE_IMAGE} AS builder

# Add our source code.
COPY --chown=rust:rust . ./

# Build our application.
RUN cargo build --release

# Now, we need to build our _real_ Docker container, copying in `rent-scrapper`.
FROM alpine
RUN apk --no-cache add ca-certificates
COPY --from=builder \
    /home/rust/src/target/x86_64-unknown-linux-musl/release/rent-scrapper \
    /usr/local/bin/
CMD /usr/local/bin/rent-scrapper
