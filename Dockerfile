FROM postgres:16

RUN apt update && apt -y install \
    curl \
    git \
    libclang-dev \
    build-essential \
    libreadline-dev \
    zlib1g-dev \
    flex \
    bison \
    libxml2-dev \
    libxslt-dev \
    libssl-dev \
    libxml2-utils \
    xsltproc \
    ccache \
    pkg-config

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# RUN chmod +x $HOME/.cargo/env && $HOME/.cargo/env
ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo install --locked cargo-pgrx && cargo pgrx init

WORKDIR /source

# RUN git clone https://github.com/txpipe/mumak.git
# WORKDIR /source/mumak/extension

COPY ./extension ./
RUN cargo pgrx install

COPY ./init-db.sh /docker-entrypoint-initdb.d/