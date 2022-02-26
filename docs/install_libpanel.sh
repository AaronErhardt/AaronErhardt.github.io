#!/bin/bash

# Return on error
set -e

git clone --depth 1 https://gitlab.gnome.org/chergert/libpanel

cd libpanel

meson _build --prefix=/usr -Dvapi=false
ninja -C _build install

cd ..
