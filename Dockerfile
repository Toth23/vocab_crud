FROM rust:1.74-buster AS builder

WORKDIR /usr/src/vocab-crud
COPY . .

RUN cargo build --release


FROM debian:buster-slim

RUN apt-get update && apt-get install -y libpq-dev postgresql && rm -rf /var/lib/apt/lists/*

COPY --from=builder /usr/src/vocab-crud/target/release/vocab_crud /usr/local/bin/vocab_crud

ENV PORT 8080

EXPOSE 8080
CMD ["vocab_crud"]
