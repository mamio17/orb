FROM orb_base

RUN mkdir -p /opt/orb
WORKDIR /opt/orb

COPY ./Cargo.toml /opt/orb/Cargo.toml
COPY ./src /opt/orb/src
ADD ./AbJo_Drum_and_Freak.wav /opt/orb/assets/sample.wav

RUN cargo build

