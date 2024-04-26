
ARG RUST_VERSION=1.76.0
ARG APP_NAME=main_bot


FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
WORKDIR /app
COPY src/ /app/src
COPY Cargo.toml /app
RUN apk add --no-cache clang lld musl-dev git

RUN cargo build --release && \
    cp ./target/release/$APP_NAME /bin/main_bot

CMD ["/bin/main_bot"]
