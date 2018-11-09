# Rust
FROM rust:slim

RUN id && pwd && ls -lha
RUN apt-get update
RUN apt-cache search sound | grep alsa
RUN cargo -V
RUN apt-get install -y libsndfile1-dev libopenal-dev
CMD tail -f /dev/null
