# Rust
FROM rust:slim

RUN id && pwd && ls -lha
RUN apt-get update
RUN apt-cache search sound | grep alsa
RUN cargo -V

CMD tail -f /dev/null
