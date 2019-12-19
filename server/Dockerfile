FROM rust AS build
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update -y && apt install -y --no-install-recommends musl-tools

WORKDIR /src
COPY . .
RUN cargo build --target x86_64-unknown-linux-musl --release
RUN cp target/x86_64-unknown-linux-musl/release/sprokit-server /sprokit-server

FROM scratch

COPY --from=build /sprokit-server /sprokit-server

EXPOSE 4000

ENTRYPOINT ["/sproket-server"]
