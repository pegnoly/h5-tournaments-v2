FROM rust:1.94.0

RUN apt-get update && apt-get install -y protobuf-compiler

WORKDIR /app
COPY . .

RUN cd shared && cargo build --release
RUN cd service && cargo build --release

EXPOSE 50051
CMD ["./service/target/release/service"]