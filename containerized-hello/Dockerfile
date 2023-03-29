FROM rust:latest as builder
ENV APP containerized-hello
WORKDIR /usr/src/$APP
COPY . .
RUN cargo install --path .
# COPY . .
# RUN cargo build --release
# CMD ["./target/release/containerized-hello"]

FROM debian:buster-slim
RUN apt-get update && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/$APP /usr/local/bin/$APP
#export this actix web service to port 8080 and 0.0.0.0
EXPOSE 8080
CMD ["containerized-hello"]