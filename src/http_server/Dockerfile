FROM rustlang/rust:nightly-slim

RUN apt update && apt install -y libpq-dev

COPY . /app

WORKDIR /app

RUN cargo build

ENTRYPOINT ["sh", "/app/entrypoint.sh"]