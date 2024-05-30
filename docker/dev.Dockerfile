FROM postgres:16

ARG UID=1000
ARG GID=1000
RUN usermod -u $UID postgres && groupmod -g $GID postgres
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
    pkg-config \
    sudo

# Temporary to make vscode extension work since its running under root
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Add postgres to the sudoers with no password prompt for specific commands
RUN echo "postgres ALL=(ALL) NOPASSWD: ALL" > /etc/sudoers.d/postgres

RUN chown -R postgres:postgres /usr/share/postgresql
RUN chown -R postgres:postgres /usr/lib/postgresql
# Using su instead of USER since dev container doesn't seem to like USER docker directive
RUN su - postgres -c 'curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y'

RUN echo 'export PATH="/var/lib/postgresql/.cargo/bin:${PATH}"' >> /var/lib/postgresql/.bashrc
RUN echo 'export USER=postgres' >> /var/lib/postgresql/.bashrc

RUN su - postgres -c 'cargo install --locked cargo-pgrx@0.11.3 && cargo pgrx init'

WORKDIR /source
COPY ./extension ./
RUN sudo chown -R postgres:postgres /source
RUN su - postgres -c 'cd /source && cargo pgrx install'

COPY ./init-db.sh /docker-entrypoint-initdb.d/