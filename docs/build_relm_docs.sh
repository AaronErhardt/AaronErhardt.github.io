#!/bin/sh

# Return on error
set -e


rm -rf relm4
mkdir relm4

mkdir tmp
cd tmp

git clone https://github.com/AaronErhardt/relm4 ./

find -name "lib.rs" -exec ../append_doc_feature.sh {} +

cargo update

cd relm4-components
cargo +nightly doc --all-features -Z rustdoc-scrape-examples=examples

cd ../relm4-macros
cargo +nightly doc --all-features -Z rustdoc-scrape-examples=examples

cd ..
cargo +nightly doc --all-features -Z rustdoc-scrape-examples=examples

cd ..

mv tmp/target/doc/* relm4

rm -rf tmp
