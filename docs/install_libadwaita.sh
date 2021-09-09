#!/bin/bash

git clone https://gitlab.gnome.org/GNOME/libadwaita
git checkout fcf62885bb28ec1da95d209c7de4bdaa3aef8746

cd libadwaita

meson . _build
ninja -C _build
ninja -C _build install

cd ..