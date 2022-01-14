#!/bin/bash

# Return on error
set -e

git clone --depth 1  https://gitlab.gnome.org/GNOME/libadwaita

cd libadwaita

meson _build --prefix=/usr -Dtests=false -Dexamples=false -Dvapi=false -Dintrospection=disabled
ninja -C _build install

cd ..
