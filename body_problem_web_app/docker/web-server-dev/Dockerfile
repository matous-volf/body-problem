# 1.88.0-bookworm
FROM rust@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0

RUN cargo install trunk --version 0.21.14

RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

CMD ["trunk", "serve", "--address", "0.0.0.0"]
