FROM rust:1.72-bookworm as builder

ARG TARGETPLATFORM
ARG DEBUG

WORKDIR /app

RUN apt-get update \
    && apt-get upgrade -y

COPY ./services .

RUN --mount=type=cache,target=/usr/local/cargo/registry,id=${TARGETPLATFORM} --mount=type=cache,target=/app/target,id=${TARGETPLATFORM} \
    cargo build && \
    mv /app/target/${DEBUG}/users /users && \ 
    mv /app/target/${DEBUG}/appointments /appointments


FROM debian:bookworm-slim AS users
COPY --from=builder /users /app/users

ENTRYPOINT /app/users

FROM debian:bookworm-slim AS appointments
COPY --from=builder /users /app/appointments

ENTRYPOINT /app/appointments
