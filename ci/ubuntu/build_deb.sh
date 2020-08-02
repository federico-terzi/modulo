#!/bin/bash

echo "Testing modulo..."
cd modulo
cargo test --release

ls

echo "Building modulo and packaging deb"
cargo deb -p modulo

cd ..
cp modulo/target/debian/modulo*.deb /shared/modulo-debian-amd64.deb

echo "Building binary"
pushd modulo
cargo build --release
popd

cp modulo/target/release/modulo /shared/modulo-linux-amd64

ls /shared