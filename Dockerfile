# CORREÇÃO 1: Adicionado o "AS builder" aqui para o segundo estágio conseguir ler os arquivos
FROM rust:bookworm AS builder

WORKDIR /app

COPY Cargo.toml Cargo.lock ./

RUN mkdir src && echo "fn main() {}" > src/main.rs 
RUN cargo build --release
RUN rm -rf src

COPY ./src ./src

RUN touch src/main.rs
RUN cargo build --release 

FROM debian:bookworm-slim
WORKDIR /app

# CORREÇÃO 2: Instalação de certificados para o 'reqwest' não quebrar ao acessar a internet
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Agora o --from=builder vai funcionar perfeitamente
COPY --from=builder /app/target/release/rust-ascii-generator /app/rust-ascii-generator

ENTRYPOINT ["./rust-ascii-generator"]
