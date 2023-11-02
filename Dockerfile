FROM rust:1.73 as builder

RUN USER=root cargo new --bin kuna-web-api
WORKDIR ./kuna-web-api
COPY ./Cargo.toml ./Cargo.toml
RUN cargo build --release

ADD . ./

RUN cargo build --release

FROM debian:latest
ARG APP=/usr/src/app

RUN apt-get update

EXPOSE 4000

ENV TZ=Etc/UTC \
    APP_USER=appuser

RUN groupadd $APP_USER \
    && useradd -g $APP_USER $APP_USER \
    && mkdir -p ${APP}

COPY --from=builder /kuna-web-api/target/release/kuna-web-api ${APP}/kuna-web-api

RUN chown -R $APP_USER:$APP_USER ${APP}

USER $APP_USER
WORKDIR ${APP}

CMD ["./kuna-web-api"]