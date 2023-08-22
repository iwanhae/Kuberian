FROM rust:1.71-bullseye as builder
WORKDIR /usr/src/kuberian

# Install intel-mkl
RUN wget -O- https://apt.repos.intel.com/intel-gpg-keys/GPG-PUB-KEY-INTEL-SW-PRODUCTS.PUB \
    | gpg --dearmor | tee /usr/share/keyrings/oneapi-archive-keyring.gpg > /dev/null && \
    echo "deb [signed-by=/usr/share/keyrings/oneapi-archive-keyring.gpg] https://apt.repos.intel.com/oneapi all main" | \
    tee /etc/apt/sources.list.d/oneAPI.list
RUN apt update
RUN apt install -y intel-oneapi-mkl-devel libomp-dev

# Make cache layers
COPY Cargo.toml .
COPY ci src/
RUN cargo build -r -F mkl
RUN rm -rf src

# Build final binaries
COPY . .
# to make sure that cargo recompile the kuberian
RUN touch src/* 
RUN cargo install --path . -F mkl

# Downloads Embedding Models or ETC
ENV HF_HOME=/model
RUN kuberian --ci

FROM debian:11 as sqlite-fetcher
RUN apt update
RUN apt-get install -y sqlite3

FROM gcr.io/distroless/cc-debian11
WORKDIR /app
COPY --from=sqlite-fetcher /usr/lib/x86_64-linux-gnu/libsqlite3.so.* /usr/lib/
COPY --from=builder /opt/intel/oneapi/compiler/latest/linux/compiler/lib/intel64_lin/libiomp5.so /usr/lib/
COPY --from=builder /model /model
COPY --from=builder /usr/local/cargo/bin/kuberian /usr/local/bin/kuberian
ADD https://s3.iwanhae.kr/kuberian/kuberian.db ./kuberian.db
ADD https://s3.iwanhae.kr/kuberian/kuberian.usearch ./kuberian.usearch
EXPOSE 8080
ENV HF_HOME=/model
CMD ["kuberian"]