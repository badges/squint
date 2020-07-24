#!/bin/bash

CARGO_BUILD_MODE="${1:-debug}"
SERVER_BINARY_NAME="${2:-squint}"

if [ "$CARGO_BUILD_MODE" == "release" ]; then
  rm -rf target/release/deps/${SERVER_BINARY_NAME}*
else
  rm -rf target/debug/deps/${SERVER_BINARY_NAME}*
fi
