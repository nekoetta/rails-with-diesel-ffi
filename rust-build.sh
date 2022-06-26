#!/bin/sh
docker run -it --rm -v $PWD/ffi/diesel_ffi:/usr/src -w /usr/src rust:latest cargo build --release
