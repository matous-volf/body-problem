FROM rust:1.80 as builder_app

RUN apt update \
  && apt install -y brotli \
  && apt clean \
  && rm -rf /var/lib/apt/lists/* /var/cache/apt/archives/*

RUN cargo install trunk
RUN rustup target add wasm32-unknown-unknown

COPY ./ /srv/app/
WORKDIR /srv/app/body_problem_web_app

RUN trunk build --release

RUN find dist/ \( -name "*.js" -o -name "*.css" -o -name "*.html" \) -exec brotli -6 -o {}.br {} \;

FROM ubuntu:22.04 as builder_nginx

RUN apt update \
    && apt upgrade -y \
    && apt install -y libpcre3 libpcre3-dev zlib1g zlib1g-dev openssl libssl-dev wget git gcc make libbrotli-dev

WORKDIR /app
RUN wget https://nginx.org/download/nginx-1.27.0.tar.gz && tar -zxf nginx-1.27.0.tar.gz
RUN git clone --recurse-submodules -j8 https://github.com/google/ngx_brotli
RUN cd nginx-1.27.0 && ./configure --with-compat --add-dynamic-module=../ngx_brotli \
    && make modules

FROM nginx:1.27.0
COPY --from=builder_nginx /app/nginx-1.27.0/objs/ngx_http_brotli_static_module.so /etc/nginx/modules/

COPY ./body_problem_web_app/docker/web-server-prod/nginx.conf /etc/nginx/nginx.conf
COPY ./body_problem_web_app/docker/web-server-prod/default.conf /etc/nginx/conf.d/default.conf

COPY --from=builder_app /srv/app/body_problem_web_app/dist/ /var/www/html/public/
