FROM debian:bullseye

# common packages
RUN apt-get update && \
    apt-get install --no-install-recommends -y \
    build-essential \
    autoconf automake autotools-dev libtool xutils-dev \
    ca-certificates curl file && \
    rm -rf /var/lib/apt/lists/*

# install toolchain
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | \
    sh -s -- --default-toolchain nightly --target wasm32-unknown-unknown -y

ENV PATH=/root/.cargo/bin:$PATH  

# install cargo binstall to reduce image size
WORKDIR /root/.cargo/bin
RUN curl -L --output cargo-binstall.tgz https://github.com/cargo-bins/cargo-binstall/releases/download/v0.19.3/cargo-binstall-x86_64-unknown-linux-gnu.tgz && \
    tar -xvzf cargo-binstall.tgz && \
    chmod +x cargo-binstall && \
    rm cargo-binstall.tgz

#RUN rustup component add rust-analyzer rustfmt rust-src clippy
RUN cargo binstall trunk@0.16.0 -y

WORKDIR /root
