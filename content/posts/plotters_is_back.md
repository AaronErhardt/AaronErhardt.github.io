---
title: "Plotters is back!"
date: 2022-04-27
# weight: 1
tags: ["plotters"]
author: "Aaron Erhardt"
showToc: false
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
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/tree/blog/content/posts"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

This is a short update on my recent [post regarding the plotters crate](https://aaronerhardt.github.io/blog/posts/plotters_future/).
The maintainer of plotters did respond and fortunately all plans for forking the project are dropped.

Now all repositories live in the [plotters-rs organization](https://github.com/plotters-rs) and can be maintained by members of the community.
Also, we're looking for people who want to join the maintenance team ([#345](https://github.com/plotters-rs/plotters/issues/345)).

## What can we learn from this?

Open source projects often start as personal projects.
Sometimes they grow bigger over time.
People start using and depending on the project, yet still only a single person has all the responsibility.
Often unexpected things happen in our lives and projects are suddenly left unmaintained.

As I said in my previous post, there nothing wrong with abandoning a project.
You are not responsible for maintaining your own projects for the rest of your life.
Nevertheless it is nice to keep a project alive and to give the community a chance to take over the development.

In fact, there are two simple steps for maintainers of open source projects:

+ Add the [Rust Bus project](https://users.rust-lang.org/t/bus-factor-1-for-crates/17046) to the owners of your crate and your repository.
  This will allow others to maintain your project in case you go inactive for several months.

+ Get the community involved as early as possible and allow members of the community to take over responsibility.
  You can also form an organization and invite others to join.
  A group chat on Matrix, Discord, Slack, etc. can also be helpful to keep in touch with the community.
  If one person leaves the others should be able to continue.
