# 1.88.0-bookworm
FROM rust@sha256:af306cfa71d987911a781c37b59d7d67d934f49684058f96cf72079c3626bfe0 as builder_app

RUN apt update \
  && apt install -y brotli \
  && apt clean \
  && rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

RUN cargo install trunk --version 0.21.14
RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

RUN trunk build --release

RUN find dist/ \( -name "*.js" -o -name "*.css" -o -name "*.html" \) -exec brotli -6 -o {}.br {} \;

# jammy-20250714
FROM ubuntu@sha256:1ec65b2719518e27d4d25f104d93f9fac60dc437f81452302406825c46fcc9cb as builder_nginx

RUN apt update \
    && apt upgrade -y \
    && apt install -y libpcre3 libpcre3-dev zlib1g zlib1g-dev openssl libssl-dev wget git gcc make libbrotli-dev

WORKDIR /app
RUN wget https://nginx.org/download/nginx-1.27.5.tar.gz && tar -zxf nginx-1.27.5.tar.gz
RUN git clone --recurse-submodules -j8 https://github.com/google/ngx_brotli
RUN cd nginx-1.27.5 && ./configure --with-compat --add-dynamic-module=../ngx_brotli \
    && make modules

# 1.27.5-bookworm
FROM nginx@sha256:6784fb0834aa7dbbe12e3d7471e69c290df3e6ba810dc38b34ae33d3c1c05f7d
COPY --from=builder_nginx /app/nginx-1.27.5/objs/ngx_http_brotli_static_module.so /etc/nginx/modules/

COPY ./body_problem_web_app/docker/web-server-prod/nginx.conf /etc/nginx/nginx.conf
COPY ./body_problem_web_app/docker/web-server-prod/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder_app /srv/app/body_problem_web_app/dist/ /var/www/html/public/
