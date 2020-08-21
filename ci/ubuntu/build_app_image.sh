#!/bin/bash

set -e

echo "Testing modulo..."
cd modulo
cargo test --release

ls

echo "Building binary"
pushd modulo
cargo build --release
popd

echo "Downloading linuxdeploy to create AppImage"

wget https://github.com/linuxdeploy/linuxdeploy/releases/download/continuous/linuxdeploy-x86_64.AppImage

# make them executable
chmod +x linuxdeploy*.AppImage 

echo "Building AppImage"

./linuxdeploy-x86_64.AppImage --appimage-extract-and-run -e "/modulo/target/release/modulo" -d "/modulo/packaging/linux/modulo.desktop" -i "/modulo/packaging/linux/icon.svg" --appdir AppDir --output appimage

cp ./modulo-*.AppImage /shared/modulo-x86_64.AppImage

ls /shared