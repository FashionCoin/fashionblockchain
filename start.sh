#!/usr/bin/env bash

cd /var/git/fashionblockchain

./target/release/node run --node-config=./out/validators/0.toml --db-path=./out/db/ --public-api-address=185.26.99.217:8181 --private-api-address=185.26.99.217:8182
