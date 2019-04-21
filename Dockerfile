FROM rust:1.33

WORKDIR /problems

COPY . /problems

RUN cargo build
