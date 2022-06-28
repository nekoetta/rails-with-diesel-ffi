#!/bin/sh

if [ ! -d "/usr/src/ffi/diesel_ffi/target" ]; then
    cd /usr/src/ffi/diesel_ffi
    $HOME/.cargo/bin/cargo build --release

    cd /usr/src
    bin/rails db:migrate;
    bin/rails runner lib/rust_seed.rb;
fi

if [ ! -d "/usr/src/public/packs" ]; then
    cd /usr/src
    yarn install;
fi

exec $@
