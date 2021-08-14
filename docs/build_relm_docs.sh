#!/bin/sh

rm -rf relm4
mkdir relm4

mkdir tmp
cd tmp

git clone https://github.com/AaronErhardt/relm4 ./

cargo update
cargo doc --features all

cd relm4-macros
cargo doc

cd ../relm4-components
cargo doc

cd ../..

mv tmp/target/doc/* relm4

rm -rf tmp
