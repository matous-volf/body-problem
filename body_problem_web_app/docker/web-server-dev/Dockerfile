FROM rust:1.80

RUN cargo install trunk

RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

CMD ["trunk", "serve", "--address", "0.0.0.0"]
