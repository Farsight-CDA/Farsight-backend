FROM rust:1.62.0-bullseye as build

WORKDIR app

COPY ./src ./src
COPY ./.git ./.git
COPY ./Cargo.lock ./
COPY ./Cargo.toml ./
COPY ./types_output ./types_output

RUN apt clean
RUN apt-get update --allow-releaseinfo-change -y
RUN apt upgrade -y

# Build your program for release
RUN cargo build --release

RUN mv target/release/farsight-backend .

FROM debian:bullseye

WORKDIR app

RUN apt update --allow-releaseinfo-change -y
RUN apt upgrade -y
RUN apt install python pip -y

RUN pip install pillow

COPY ./SourceCodePro-Bold.ttf .
COPY ./background.png .
COPY ./python .
COPY --from=build /app/farsight-backend .

# Run the binary
CMD ["./farsight-backend"]

