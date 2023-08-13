FROM rust:1.71 as builder
WORKDIR /usr/src/kuberian
ENV HF_HOME=/model
COPY Cargo.toml .
COPY ci src/
RUN cargo run --release
RUN rm -rf src
COPY . .
RUN cargo install --path .

FROM debian:bullseye-slim
ENV HF_HOME=/model
COPY --from=builder /model /model
COPY --from=builder /usr/local/cargo/bin/kuberian /usr/local/bin/kuberian
EXPOSE 8080
CMD ["kuberian"]