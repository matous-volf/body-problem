FROM rust:1.88.0-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

RUN cargo install --locked trunk --version 0.21.14
RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

RUN trunk build --release

FROM nginx/nginx-unprivileged:1.30.6-alpine3.24-slim@sha256:984a21e29bb10d4c0e122374b97b89a2101823a9331e0f3c6c04bc285ccf447f

COPY ./body_problem_web_app/docker/web-server-prod/nginx.conf /etc/nginx/nginx.conf
COPY ./body_problem_web_app/docker/web-server-prod/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /srv/app/body_problem_web_app/dist/ /var/www/html/public/
