# Build Stage 
FROM rust:bookworm AS builder

WORKDIR /app

ARG DATABASE_URL

ENV DATABASE_URL=${DATABASE_URL}

COPY . .

RUN cargo build --release 

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /user/local/bin

COPY --from=builder /app/target/release/personal_finance_tracker .

CMD ["./personal_finance_tracker"]