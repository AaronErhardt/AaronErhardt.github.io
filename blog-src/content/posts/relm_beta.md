---
title: "Relm4 beta released 🎉"
date: 2021-08-11T15:00:00+30:00
# weight: 1
# aliases: ["/relm4_beta"]
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

## What's [relm](https://github.com/antoyo/relm)?

Relm is described on its repository as an "idiomatic, GTK+-based, GUI library, inspired by Elm, written in Rust" and has more than 2000 GitHub stars and over 36.000 downloads on [crates.io](https://crates.io/crates/relm).

When I first started with GUI development in Rust and experimented with [gtk-rs](https://github.com/gtk-rs/gtk3-rs) I had a rough time because I was completely new to GUI development and GTK. Relm on the other side provided me the examples and structure I needed to get productive in just a couple of hours. Over the next weeks I continued using relm to work on an application I write at my job and was very pleased with the productivity I was able to archive.

When GTK4 and a few weeks later [gtk4-rs](https://github.com/gtk-rs/gtk4-rs) were released I was eager to use relm with this latest version of GTK to quickly update the dependencies of my application. However, it turned out that relm was not easy to port but it relied on features - most importantly the [ContainerExt trait](https://gtk-rs.org/gtk3-rs/stable/latest/docs/gtk/prelude/trait.ContainerExt.html) - that were removed with GTK4.

## The story of Relm4

Naturally, I started thinking about how to solve this issue and get relm working with GTK4. I quickly came up with several ideas and reached out for feedback by the community. Yet while hacking on relm I was quickly overwhelmed by its complexity but slowly realized how the underlying mechanisms worked. With this knowledge I started from scratch with a minimal version of Relm4.

Over time I added many features and rewrote the core of the library dozens of times to fit all requirements. Many examples were added to test out the new features and to expose limitations.

Now finally after weeks of work, I'm satisfied with the result and released the first beta version of Relm4.

## Where to get started

I'm not going into depth about the changes and the new API in this post but there's a new book I'm working on, the docs and a many examples to get started.

+ **[Repository](https://github.com/AaronErhardt/relm4)**
+ 📖 **[Book](https://aaronerhardt.github.io/relm4-book/book/)**
+ **[Rust documenation](https://aaronerhardt.github.io/docs/relm4/relm4/)**

## Conclusion

I hope that Relm4 can offer a pleasant experience for new users and those who want to port their existing relm apps to GTK4. I believe that gtk4-rs offers the most complete and mature GUI experience for Rust programmers right now and that Relm4 will help new and experienced users to productively write efficient GUI applications.

I highly appreciate feedback and contributions on Relm4 and thank those who helped me so far, most importantly [antoyo](https://github.com/antoyo), [MGlolenstine](https://github.com/MGlolenstine) and the whole gtk-rs team for providing awesome Rust bindings for GTK.