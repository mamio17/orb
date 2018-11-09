# Rust
FROM rust:slim

RUN apt-get update
RUN apt-get install -y libsndfile1-dev libopenal-dev alsa-utils
RUN cargo -V

CMD tail -f /dev/null
