FROM ekidd/rust-musl-builder:nightly-2019-09-05-openssl11 as builder
ADD src/ ./src
COPY Cargo.lock Cargo.toml ./

RUN cargo build --release 

FROM scratch
COPY --from=gcr.io/berglas/berglas:latest /bin/berglas /bin/berglas
COPY --from=builder /etc/ssl/certs/ca-certificates.crt /etc/ssl/certs/ca-certificates.crt
COPY --from=builder home/rust/src/target/x86_64-unknown-linux-musl/release/dappface-api ./app
ENV PORT 8080

ENTRYPOINT "exec /bin/berglas exec -- ./app"
