ARG SERVER_BINARY_NAME=squint
ARG CARGO_BUILD_MODE=debug

FROM rust:1.55.0 as builder
ARG SERVER_BINARY_NAME
ARG CARGO_BUILD_MODE

RUN if [ "$CARGO_BUILD_MODE" != "release" ] && [ "$CARGO_BUILD_MODE" != "debug" ]; then \
      echo "Invalid value for CARGO_BUILD_MODE build arg: '$CARGO_BUILD_MODE'. Must be 'debug' or 'release'" \
      exit 1; \
    fi
RUN apt update && apt install -y libgtk-3-dev

WORKDIR /usr/src/${SERVER_BINARY_NAME}
# Leverage docker build cache for dependencies
COPY Cargo.toml \
    Cargo.lock \
    scripts/docker/build.sh \
    scripts/docker/reset.sh \
    ./

RUN mkdir src/
# This is only used to avoid unnecessarily recompiling dependencies every time
# and will be replaced with the actual application. The message here would only
# show when attempting to start a container with a version of the image that didn't
# successfully build.
RUN echo "fn main() { println!(\"Docker image build error\") }" > src/main.rs
RUN ./build.sh ${CARGO_BUILD_MODE}
RUN ./reset.sh ${CARGO_BUILD_MODE} ${SERVER_BINARY_NAME}

# Run the main build
COPY . .
RUN ./build.sh ${CARGO_BUILD_MODE}

FROM ubuntu:focal
ARG SERVER_BINARY_NAME
ARG CARGO_BUILD_MODE
ENV SERVER_BINARY_NAME=${SERVER_BINARY_NAME}

RUN echo "ttf-mscorefonts-installer msttcorefonts/accepted-mscorefonts-eula select true" | debconf-set-selections
RUN echo 'debconf debconf/frontend select Noninteractive' | debconf-set-selections
RUN apt update -y && apt install -y libgtk-3-dev ttf-mscorefonts-installer
RUN apt install -y fonts-noto


COPY --from=builder /usr/src/${SERVER_BINARY_NAME}/target/${CARGO_BUILD_MODE}/${SERVER_BINARY_NAME} /usr/local/bin/${SERVER_BINARY_NAME}
CMD ["sh", "-c", "$SERVER_BINARY_NAME"]
