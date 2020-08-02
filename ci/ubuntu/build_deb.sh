#!/bin/bash

echo "Testing modulo..."
cd modulo
cargo test --release

echo "Building modulo and packaging deb"
cargo deb -p modulo

cd ..
cp modulo/target/debian/modulo*.deb /shared/modulo-debian-amd64.deb

echo "Copying to mounted volume"
cp modulo-debian-* /shared

echo "Building binary"
cargo build --release
cp modulo/target/release/modulo /shared/modulo-linux-amd64

ls /shared