#!/bin/sh
#set -e

if [[ $# -ne 1 ]]; then
	echo "usage: $0 image:tag"
	exit 1
fi

which blender
if [[ $? -ne 0 ]]; then
	echo "blender not found, cannot build"
	exit 1
fi
which cargo
if [[ $? -ne 0 ]]; then
	echo "cargo not found, cannot build"
	exit 1
fi
./render_all.sh
cd ../bot
cargo build --release
cd -
cp ../bot/target/release/adachi-bot adachi-bot
docker build -t "$1" .
rm adachi-bot
