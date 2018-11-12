FROM orb_base

ENV ORB_DIR /opt/orb

RUN mkdir -p ${ORB_DIR}
WORKDIR ${ORB_DIR}
COPY ./ ${ORB_DIR}/
VOLUME ${ORB_DIR}

RUN cargo build
