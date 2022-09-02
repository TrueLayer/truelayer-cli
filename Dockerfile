FROM rust:1.61

WORKDIR /usr/src/app
COPY . .

RUN cargo install --path .

ENTRYPOINT ["truelayer"]
