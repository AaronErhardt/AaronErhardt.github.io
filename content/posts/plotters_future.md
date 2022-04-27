---
title: "Planning the future of plotters"
date: 2022-04-12
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
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/tree/blog/content"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

With over 1900 stars on GitHub and over 600.000 downloads per month, [plotters](https://crates.io/crates/plotters) is a very prominent and widely used Rust crate.
By a huge margin it is the most used plotting library for Rust and covers a lot of different use-cases and back-ends.

Yet, since roughly June 2021, almost 10 months ago, there hasn't been any considerable activity on the main branch of the repository.
At the same time, the community has been very active with 22 pull requests submitted since then, but none of them was reviewed or merged.
Everything looks like the project was discontinued by its original creator.

That said, there's nothing wrong with abandoning a project.
Of course, the creator of plotters has every right to put his focus on other projects and discontinue the development of plotters.
However with a project as popular as plotters, it not a great situation if you leave without allowing the community to continue the development.

Sadly, I wasn't able to contact the creator on GitHub or via email.
I don't know the backstory and don't want to speculate why this is happening.
All I know is that the plotters community is in an awkward situation right now.
As long as the owner of the repository and the crates.io package doesn't respond they need to fork the repository and find a new name for the crate.
Unfortunately, this seems to be the only option.

## How to proceed

First of all, if anybody knows how to reach the creator, please let us know.
Ideally we can still contact him to continue the development of the crate in the same repository and under the same name.

If that doesn't work, I suggest to fork the repository and plan all further steps from there.

I offer to step up as an interim maintainer.
Likely, I won't contribute a lot of code as I'm occupied with other projects but I can review and merge pull requests.
I'd be very happy if others would consider a role as maintainer, too.

FYI, the [maintenance issue at the plotter repo](https://github.com/38/plotters/issues/333).
