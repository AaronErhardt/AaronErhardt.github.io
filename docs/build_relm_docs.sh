#!/bin/sh

rm -rf relm4
mkdir relm4

mkdir tmp
cd tmp

git clone https://github.com/AaronErhardt/relm4 ./

find -name "lib.rs" -exec ../append_doc_feature.sh {} +

cargo update

cd relm4-components
cargo +nightly doc --all-features

cd ../relm4-macros
cargo +nightly doc --all-features

cd ..
cargo +nightly doc --all-features

cd ..

mv tmp/target/doc/* relm4

git add relm4/*

rm -rf tmp
