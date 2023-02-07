FROM rust:1.66.0-buster as chef
RUN cargo install cargo-chef
WORKDIR /app

FROM chef as planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef as builder
COPY --from=planner /app/recipe.json recipe.json
COPY Cargo.* /app/
RUN rustup target add wasm32-unknown-unknown
RUN cargo chef cook --recipe-path recipe.json --release
RUN cargo install --locked trunk wasm-bindgen-cli
COPY . /app
RUN trunk build --release yew-test
