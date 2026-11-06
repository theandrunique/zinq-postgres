FROM node:22-alpine AS frontend-build

ARG BUILD_MODE=production

WORKDIR /app
COPY frontend/package*.json ./
RUN npm ci
COPY frontend/ ./
RUN npm run build -- --mode ${BUILD_MODE}


FROM rust:1.97.1-alpine AS backend-build

RUN apk add --no-cache openssl-dev pkgconf

WORKDIR /app
COPY Cargo.toml Cargo.lock ./

# Build dependencies against a stub entry point so dependency compilation
# stays cached when only the application source changes.
RUN mkdir -p src scripts \
    && echo 'fn main() {}' > src/main.rs \
    && echo 'fn main() {}' > scripts/test.rs \
    && cargo build --release --locked --bin zinq \
    && rm -rf src scripts

COPY . ./
RUN touch src/main.rs \
    && cargo build --release --locked --bin zinq


FROM alpine:3.22

WORKDIR /app
RUN apk add --no-cache ca-certificates libgcc openssl

COPY --from=backend-build /app/target/release/zinq ./zinq
COPY --from=frontend-build /app/build ./static
COPY scripts/docker-entrypoint.sh /usr/local/bin/docker-entrypoint.sh
RUN chmod +x /usr/local/bin/docker-entrypoint.sh

ENV PORT=8080
EXPOSE 8080

ENTRYPOINT ["/usr/local/bin/docker-entrypoint.sh"]
CMD ["./zinq"]
