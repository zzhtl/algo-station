FROM golang:1.26.3-alpine3.23

RUN addgroup -g 65532 -S judge && adduser -u 65532 -S -G judge -h /tmp judge

ENV CGO_ENABLED=0 \
    GO111MODULE=off \
    HOME=/tmp \
    GOCACHE=/tmp/go-cache

USER 65532:65532
WORKDIR /workspace

