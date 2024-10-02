FROM rust:alpine
WORKDIR /var/lib/maritime_pilot
ADD Cargo.* ./
RUN apk update && \
    apk upgrade && \
    apk add --no-cache \
        gcc \
        musl-dev \
        postgresql-dev
RUN apk --no-cache add libpq
ADD . .
RUN source .env
CMD cargo run
