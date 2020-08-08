FROM alpine:latest AS base
WORKDIR /app
EXPOSE 80

FROM rust:alpine3.11 as build
WORKDIR /src
COPY . .
RUN cargo build --release

FROM base AS final
WORKDIR /app
COPY --from=build /src/target/release/ .
ENTRYPOINT ["./http-debug"]
