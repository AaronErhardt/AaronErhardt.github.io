---
title: "Announcing Relm4 v0.2"
date: 2021-10-09T15:00:00+00:00
# weight: 1
# aliases: ["/relm4_v0.2_release"]
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
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/blob/blog/content/"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

Roughly one month after the first stable release a I'm happy to announce the second stable release of Relm4! 🚀🚀🚀

## About Relm4

We believe that GUI development should be easy, productive and delightful.  
The [gtk4-rs](https://github.com/gtk-rs/gtk4-rs) crate already provides everything you need to write modern, beautiful and cross-platform applications. Built on top of this foundation, Relm4 makes developing more idiomatic, simpler and faster and enables you to become productive in just a few hours.

### What's new?

Overall there have been a lot of fixes and improvements, most importantly:

+ Better error messages in the widget macro

+ Widgets and components are now always added in the correct order by the widget macro ([#20](https://github.com/AaronErhardt/relm4/issues/20))

+ Reusable components can now be used multiple times in the same application (thanks to [mskorkowski](https://github.com/mskorkowski))

+ Factories were reworked to better handle TabViews in libadwaita

The full change log can be found [here](https://github.com/AaronErhardt/relm4/blob/main/CHANGES.md). Even though there are a few breaking changes, existing projects should be able to update with no or only minor changes. 

## Where to get started

Relm4 has an outstanding documentation. The official book covers every aspect of Relm4 and is made for everyone from beginners to experts. Also, there a lots of examples that can be found in the repository.

+ ⭐ **[Repository](https://github.com/AaronErhardt/relm4)**
+ 📖 **[The Relm4 book](https://aaronerhardt.github.io/relm4-book/book/)**
+ 📜 **[Rust documentation](https://aaronerhardt.github.io/docs/relm4/relm4/)**

## Special thanks

Maintaining an open-source library next to a full time job is quite challenging. All the more I appreciate feedback and contributions to Relm4 and thank those who helped me with this release:

+ [mskorkowski](https://github.com/mskorkowski) who pointed out several issues and also fixed them :)
+ [tronta](https://github.com/tronta) who again contributed a lot to the book and added examples
+ [maruns](https://github.com/maruns), [bunnegirl](https://github.com/bunnegirl) and [a-kenji](https://github.com/a-kenji) who provided helpful feedback
+ the whole gtk-rs team for providing awesome Rust bindings for GTK and always being helpful