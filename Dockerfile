FROM rust:1.39

WORKDIR /app

COPY . /app

RUN cargo build --release

EXPOSE 6002

CMD ["cargo", "run", "--release"]
