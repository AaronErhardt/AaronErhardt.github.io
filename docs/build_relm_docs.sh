#!/bin/sh

rm -rf relm4
rm -rf relm4-macros
rm -rf relm4-components

mkdir relm4
mkdir relm4-macros
mkdir relm4-components

mkdir tmp
cd tmp
git clone https://github.com/AaronErhardt/relm4 ./

cargo update
cargo doc
mv target/doc/* ../relm4

cd relm4-macros
cargo doc
mv target/doc/* ../../relm4-macros

cd ../relm4-components
cargo doc
mv target/doc/* ../../relm4-components

cd ../..
rm -rf tmp
