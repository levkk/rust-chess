FROM rustlang/rust:nightly-slim

RUN apt update && apt install -y libpq-dev

COPY . /app

WORKDIR /app

RUN cargo build

COPY entrypoint.sh /app/entrypoint.sh

ENTRYPOINT ["sh", "/app/entrypoint.sh"]