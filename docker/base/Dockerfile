# Rust
FROM rust:slim
RUN apt-get update
RUN apt-get install -y libsndfile1-dev libopenal-dev pulseaudio alsa-utils mplayer && \
    apt-cache clean
RUN cargo -V
CMD tail -f /dev/null
