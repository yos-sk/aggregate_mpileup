FROM rust:1.66.0-buster
MAINTAINER Keisuke Yamamoto

WORKDIR /tools

RUN git clone https://github.com/kyamamot/aggregate_mpileup.git && \
    cd aggregate_mpileup && \
    cargo build --release



