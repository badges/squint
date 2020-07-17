#!/bin/bash

CARGO_BUILD_MODE="${1:-debug}"

if [ "$CARGO_BUILD_MODE" == "release" ]; then
  cargo build --release
else
  cargo build
fi
