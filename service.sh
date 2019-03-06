#!/bin/sh

pkill -9 node


cd /var/git/fashionblockchain
git commit -a -m "local"
git pull
nohup sh /var/git/fashionblockchain/start.sh > /var/log/fashion/blockchainlog.txt 2>&1 &

tail -f /var/log/fashion/blockchainlog.txt