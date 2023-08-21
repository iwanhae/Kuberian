FROM rust:1.71-bullseye as builder
RUN wget -O- https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB \
    | gpg --dearmor | tee /usr/share/keyrings/oneapi-archive-keyring.gpg > /dev/null && \
    echo "deb [signed-by=/usr/share/keyrings/oneapi-archive-keyring.gpg] https://apt.repos.intel.com/oneapi all main" | \
    tee /etc/apt/sources.list.d/oneAPI.list
RUN apt update
RUN apt install -y intel-oneapi-mkl-devel libomp-dev

WORKDIR /usr/src/kuberian
ENV HF_HOME=/model
COPY Cargo.toml .
COPY ci src/
RUN cargo build -r -F mkl
RUN rm -rf src
COPY . .
RUN cargo install --path . -F mkl
RUN kuberian --ci

FROM debian:bullseye-slim
RUN apt update && \ 
    apt install -y libomp-dev && \
    rm -rf /var/lib/apt/lists/*
ENV HF_HOME=/model
COPY --from=builder /model /model
COPY --from=builder /usr/local/cargo/bin/kuberian /usr/local/bin/kuberian
EXPOSE 8080
CMD ["kuberian"]