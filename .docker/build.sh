#!/bin/bash

rm -rf build/src build/Cargo.toml build/Cargo.lock
cp -r ../src build/
cp ../Cargo.toml ../Cargo.lock build/
docker build -t npcdw/simple-websocket:latest build