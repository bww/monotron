FROM ekidd/rust-musl-builder:stable as builder
COPY . .
RUN cargo build --release

FROM alpine:latest
ARG APP=/app

ENV TZ=Etc/UTC
RUN apk update \
    && apk add --no-cache ca-certificates tzdata \
    && rm -rf /var/cache/apk/*

COPY --from=builder /home/rust/src/target/x86_64-unknown-linux-musl/release/monotron ${APP}/monotron

WORKDIR ${APP}
EXPOSE 3030
CMD ["./monotron"]
