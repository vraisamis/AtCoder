#!/usr/bin/env bash

mkdir $1
cd $1
echo 1.15.1 > rust-toolchain

alpha="0abcdefghijklmnopqrstuvwxyz"
for i in `seq $2`
do
  echo "i = $i, ${alpha:$i:1}";
  cargo new --bin ${alpha:$i:1}
done

cd ..
