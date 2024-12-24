FROM lukemathwalker/cargo-chef:0.1.68-rust-1.83.0-slim-bookworm AS base
WORKDIR /app

# FROM debian:bookworm-slim AS chrome
# RUN apt -y update && apt -y upgrade && apt -y install curl jq unzip
# WORKDIR /chrome
# COPY docker-helpers/gc-dl.sh .
# RUN ./gc-dl.sh download chrome 133.0.6911.0 .
# RUN ./gc-dl.sh download chromedriver 133.0.6911.0 .
# RUN ./gc-dl.sh unzip .

FROM base AS recipe
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM recipe AS build
WORKDIR /build
COPY --from=recipe /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --bin raamattu-test --release

FROM debian:bookworm-slim AS deploy
WORKDIR /app
COPY --from=build /build/target/release/raamattu-test .
#COPY --from=chrome /chrome /app/chrome
# COPY --from=build /build/raamattu/static static
CMD ./raamattu-test

# vim: et ts=4 sw=4

