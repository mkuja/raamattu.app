# Install trunk for building and serving fe in development.
FROM rust:1-bookworm AS trunk
RUN cargo install trunk && \
    rustup target add wasm32-unknown-unknown

FROM trunk AS serve_dev
WORKDIR /app/raamattu-fe
COPY . .
EXPOSE 8080
ENTRYPOINT [ "trunk", "serve", "-a", "0.0.0.0" ]

# vim: et ts=4 sw=4
