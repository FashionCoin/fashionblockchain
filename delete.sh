#!/usr/bin/env bash

cd /var/git/fashionblockchain

pkill -9 node

rm -r /var/git/fashionblockchain/out/db
mkdir /var/git/fashionblockchain/out/db