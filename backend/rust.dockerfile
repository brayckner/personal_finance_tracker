# Build Stage 
FROM rust:bookworm AS builder

WORKDIR /app

ARG DATABASE_URL
ARG TIINGO_API_KEY

ENV DATABASE_URL=${DATABASE_URL}
ENV TIINGO_API_KEY=${TIINGO_API_KEY}

COPY . .

RUN cargo build --release 

FROM debian:bookworm-slim

RUN apt-get update && apt install -y openssl

WORKDIR /usr/local/bin

COPY --from=builder /app/target/release/personal_finance_tracker .

CMD ["./personal_finance_tracker"]