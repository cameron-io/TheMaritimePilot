FROM rust:alpine
WORKDIR /var/lib/maritime_pilot
ADD Cargo.* ./
RUN apk update && \
    apk upgrade && \
    apk add libpq-dev
RUN source .env
ADD . .
CMD cargo run
