FROM node:16-alpine3.14 as ui-builder

WORKDIR /app
COPY . .
RUN wget -q -P /tmp https://get.pnpm.io/v6.14.js && \
    node /tmp/v6.14.js add --global pnpm
RUN ["pnpm", "install", "--frozen-lockfile", "--dir", "/app/ui"]
RUN ["pnpm", "run", "build", "--dir", "/app/ui"]

FROM ekidd/rust-musl-builder:stable as ws-builder

WORKDIR /app
COPY --chown=rust:rust . .
RUN ["cargo", "build", "--release"]

FROM nginx:1-alpine

RUN ["rm", "-rf", "/usr/share/nginx/html"]
COPY --from=ui-builder /app/ui/build/ /usr/share/nginx/html
COPY --from=ws-builder /app/target/x86_64-unknown-linux-musl/release/sweeprs-web /usr/local/bin

CMD nginx && /usr/local/bin/sweeprs-web
