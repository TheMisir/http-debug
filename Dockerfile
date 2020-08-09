FROM alpine:latest AS base
EXPOSE 80

FROM base AS final
WORKDIR /app
RUN apk --no-cache add curl && \
    curl https://github.com/TheMisir/http-debug/releases/latest/download/http-debug-linux -o /app/http-debug -s && \
    chmod +x /app/http-debug
ENTRYPOINT ./http-debug
