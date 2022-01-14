#!/bin/bash

# Return on error
set -e

git clone https://gitlab.gnome.org/GNOME/libadwaita

cd libadwaita

meson . _build -Dvapi=false -Dtests=false -Dexamples=false
ninja -C _build
ninja -C _build install

cd ..
