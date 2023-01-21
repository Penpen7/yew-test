FROM rust:1.66.0-buster as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as cacher
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --recipe-path recipe.json

FROM rust:1.66.0-buster as develop
WORKDIR /app
RUN rustup target add wasm32-unknown-unknown
RUN cargo install --locked trunk
RUN rustup component add clippy rustfmt
RUN cargo install --locked cargo-watch
COPY --from=cacher /app/target target
CMD trunk serve --address 0.0.0.0 --port=8080

