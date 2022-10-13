#!/bin/sh

set -o errexit
set -o nounset
set -o pipefail
set -o xtrace

readonly TARGET_HOST=192.168.137.245
readonly TARGET_PATH=/home/mateusz/rust-led-lights
readonly SOURCE_PATH=./target/release/rust-led-lights

cargo build --release
rsync ${SOURCE_PATH} ${TARGET_HOST}:${TARGET_PATH}
ssh -t ${TARGET_HOST} ${TARGET_PATH}
