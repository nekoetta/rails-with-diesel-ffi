#!/bin/sh

if [ ! -d "/usr/src/public/packs" ]; then
    yarn install;
    bin/rails db:migrate;
    bin/rails runner lib/rust_seed.rb;
fi

exec $@