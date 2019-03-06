#!/usr/bin/env bash

cd /var/git/fashionblockchain

./target/release/node run --node-config=./out/validators/0.toml --db-path=./out/db/ --public-api-address=5.187.2.103:8181 --private-api-address=5.187.2.103:8182
