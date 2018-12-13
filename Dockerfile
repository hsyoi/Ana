FROM rust:slim AS build
COPY Cargo.toml /Ana/
COPY src/ /Ana/src
COPY .cargo/ /Ana/.config
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libzmq3-dev \
    pkg-config
RUN cd /Ana && \
    cargo build -v --release

FROM ubuntu:18.04
COPY --from=build /Ana/target/release/ana /usr/local/bin
RUN apt-get update && \
    apt-get install -y --no-install-recommends \
    libzmq3-dev \
    gcc \
    g++ && \
    apt-get clean
EXPOSE 8800 8801
CMD [ "ana" ]
