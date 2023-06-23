FROM rust

WORKDIR /app

COPY . .

RUN cargo build --release

EXPOSE 8080

CMD ["./target/release/rust-web-server-async-multi-threaded"]
