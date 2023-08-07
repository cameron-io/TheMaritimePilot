FROM rust:alpine
WORKDIR /var/lib/maritime_pilot
ADD Cargo.* ./
RUN apk update && \
    apk upgrade
RUN cargo build
ADD . .
RUN source .env
CMD cargo run
