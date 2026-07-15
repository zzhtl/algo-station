FROM rust:1.96.0-alpine3.23

RUN addgroup -g 65532 -S judge && adduser -u 65532 -S -G judge -h /tmp judge

ENV HOME=/tmp

USER 65532:65532
WORKDIR /workspace

