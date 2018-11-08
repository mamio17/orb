FROM 
RUN mkdir -p /opt/orb
WORKDIR /opt/orb

COPY Cargo.toml /opt/orb/Cargo.toml
COPY src /opt/orb/src

RUN cargo build
