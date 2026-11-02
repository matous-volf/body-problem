FROM rust:1.88.0-bookworm@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 AS builder

RUN cargo install --locked trunk --version 0.21.14
RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

RUN trunk build --release

FROM nginx/nginx-unprivileged:1.31.8-alpine3.24-slim@sha256:633f37ad70b9fd48d4da573db1ebc9111a9ad174e5178e83bd2ef45381fde5e2

COPY ./body_problem_web_app/docker/web-server-prod/nginx.conf /etc/nginx/nginx.conf
COPY ./body_problem_web_app/docker/web-server-prod/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder /srv/app/body_problem_web_app/dist/ /var/www/html/public/
