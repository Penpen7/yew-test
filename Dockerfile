FROM rust:1.66.0-buster as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
COPY Cargo.* /app/
RUN cargo chef cook --recipe-path recipe.json --release
RUN cargo install --locked trunk wasm-bindgen-cli
COPY src /app/src
RUN trunk build --release
