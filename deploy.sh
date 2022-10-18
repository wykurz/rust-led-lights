#!/bin/sh

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=192.168.0.29
readonly TARGET_PATH=/home/mateusz/rust-led-lights
readonly TARGET_ARCH=armv7-unknown-linux-musleabihf
readonly SOURCE_PATH=./target/${TARGET_ARCH}/release/rust-led-lights

cargo build --release --target=${TARGET_ARCH}
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}
