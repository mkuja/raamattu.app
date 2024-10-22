FROM lukemathwalker/cargo-chef:0.1.68-rust-1-slim-bookworm AS base
WORKDIR /app

FROM base AS recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM recipe AS build
WORKDIR /build
COPY --from=recipe /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --bins --release

FROM debian:bookworm-slim AS deploy
WORKDIR /app
COPY --from=build /build/target/release/raamattu .
COPY --from=build /build/raamattu/static static
EXPOSE 3000
CMD /app/raamattu

# vim: et ts=4 sw=4
