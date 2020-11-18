ARG RUST_BUILDER_IMAGE=ekidd/rust-musl-builder:stable

FROM $RUST_BUILDER_IMAGE as rust

ARG CARGO_BUILD_TARGET=x86_64-unknown-linux-musl
ARG RUSTRELEASEDIR="release"

WORKDIR /app/bot
RUN sudo chown -R rust:rust .
COPY . ./
RUN cargo clean
RUN cargo build --release

# reduce binary size
RUN strip ./target/$CARGO_BUILD_TARGET/$RUSTRELEASEDIR/janitor-bot

RUN cp ./target/$CARGO_BUILD_TARGET/$RUSTRELEASEDIR/janitor-bot /app/bot/

FROM alpine:latest as janitor

RUN addgroup -g 1000 janitor
RUN adduser -D -s /bin/sh -u 1000 -G janitor janitor

# Copy resources
COPY --chown=janitor:janitor --from=rust /app/bot/janitor-bot /app/janitor

RUN chown janitor:janitor /app/janitor
USER janitor
CMD ["/app/janitor"]