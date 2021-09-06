---
title: "Announcing Relm4 v0.1"
date: 2021-09-06T15:00:00+30:00
# weight: 1
# aliases: ["/relm4_release"]
tags: ["relm4"]
author: "Aaron Erhardt"
showToc: true
TocOpen: false
draft: false
hidemeta: false
comments: false
disableShare: false
disableHLJS: false
hideSummary: false
ShowReadingTime: true
ShowBreadCrumbs: true
ShowPostNavLinks: true
editPost:
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/blob/master/blog-src/content"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

Roughly one month after the first beta release and countless hours of work I'm happy to announce the first stable release of Relm4! 🎉🎉🎉

## About Relm4

Relm4 is an idiomatic GUI library inspired by [Elm](https://elm-lang.org/) and based on [gtk4-rs](https://crates.io/crates/gtk4). 
It is a new version of [relm](https://github.com/antoyo/relm) that's built from scratch and is compatible with [GTK4](https://www.gtk.org/) and [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita). The main goals are productivity, flexibility, simplicity and maintainability.

Over the last beta releases, the development slowly calmed down up to a point where I'm comfortable releasing the first stable version. I've already started porting my existing app I wrote with GTK3 and relm to use GTK4, libadwaita and Relm4 and I was very pleased with the productivity I was able to archive. I believe Relm4 offers a truly outstanding experience for GUI development in Rust.

### What's new?

+ Support for [libadwaita](https://gitlab.gnome.org/GNOME/libadwaita) was added
+ The book is now complete 🎉
+ There's a new message handler type for non-blocking IO and other use-cases
+ Many reusable components were added
+ Lots of other improvements and fixes

The full change log can be found [here](https://github.com/AaronErhardt/relm4/blob/main/CHANGES.md).

## Where to get started

Relm4 has an outstanding documentation. The official book covers every aspect of Relm4 and is made for everyone from beginners to experts. Also, there a lots of examples that can be found in the repository.

+ ⭐ **[Repository](https://github.com/AaronErhardt/relm4)**
+ 📖 **[Book](https://aaronerhardt.github.io/relm4-book/book/)**
+ 📜 **[Rust documentation](https://aaronerhardt.github.io/docs/relm4/relm4/)**

## Special thanks

I highly appreciate feedback and contributions to Relm4 and thank those who helped me with this release:

+ [tronta](https://github.com/tronta) who contributed **a lot** to the book
+ [overlisted](https://github.com/overlisted), [bunnegirl](https://github.com/bunnegirl) and [maruns](https://github.com/maruns) who provided helpful feedback
+ the whole gtk-rs team for providing awesome Rust bindings for GTK and always being helpful