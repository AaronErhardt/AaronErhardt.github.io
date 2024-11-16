---
title: "The unfortunate situation of tuxedo-rs"
date: 2024-11-16
# weight: 1
tags: ["tuxedo-rs"]
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
    URL: "https://github.com/AaronErhardt/AaronErhardt.github.io/blob/blog/content"
    Text: "Suggest Changes" # edit text
    appendFilePath: true # to append file path to Edit link
---

This week I was talking to a fellow student at my university.
He was thinking about buying a laptop to replace his aging Microsoft Surface device.
He also expressed interest in trying Linux and asked specifically which laptop I was using.
So I said: "That's a laptop made by TUXEDO Computers.
It's actually pretty cool because the company is located here in Augsburg
and they even sponsor our Formula Student team.
They ship laptops with Linux pre-installed if you want.
Also, you'll get a discount as a student."

I'd like to live in world were I could have left it there.
But unfortunately I also had to point out some problems with TUXEDO Computers in order to give him an on honest overview over what he should expect as a customer.
These problems happen to overlap with the problems I have as maintainer of tuxedo-rs at the moment.
This recent event reminded me to write a status report of the project that I had planned to do for a long time.

> DISCLAIMER: This article reflects my personal experience and understanding.
> I'm not informed about the latest internal developments at TUXEDO and I sincerely hope that they are aware of the issues and are planning to address them soon.
> Also, despite criticizing the decisions made by TUXEDO Computer, I really appreciate the company itself and they build great hardware in my opinion.
> My goal with this article is to point out the large amount of wasted potential in their software stack.

### What is tuxedo-rs

Tuxedo-rs is a community-built (not supported or connected to TUXEDO Computers) collection of Rust libraries and programs for interacting with hardware from TUXEDO Computers. 
It implements fan control and keyboard LED control from the user space among other features -
which is pretty neat if you want to set custom fan curves or want to animate your keyboard colors.
For example, I use it often to switch between quiet, power saving profiles and louder, more performant profiles depending on my situation.
Additionally, tuxedo-rs is fully modularized.
You can use it to write your own software at any abstraction level you want.

Overall, I think tuxedo-rs turned out really great.
Using Rust and building upon a robust software architecture made it possible to build great abstractions around the driver interfaces which are not only highly efficient, but also impossible to use incorrectly.
The control service and the GUI also make great use of Rust's async capabilities.
On top of that, due to clever control algorithms, tuxedo-rs tends to be quite a bit more efficient than the software provided by TUXEDO.
Unfortunately, moving forward and extending the functionality to more devices happens to be a greater challenge than anticipated.

## TUXEDO's Software Strategy: General Concerns

TUXEDO Computers provides software that offers quite similar features as tuxedo-rs (and some more).
So you might wonder why tuxedo-rs exists in the first place.
The problem it that this software only works out-of-the-box if you use their custom built Linux distribution (TUXEDO OS).
If you want to use a different distribution of Linux you're out of luck.
In this case, you have to go through the tedious process of building and installing the components yourself.

### The kernel driver

You might wonder why drivers are a problem.
After all, Linux is a collaborative project.
Anyone can contribute patches to make their device work out-of-the-box.
Yet, even after years, TUXEDO still only maintains an out-of-tree driver.
So, unless you use TUXEDO OS or some volunteer packaged it for your distro, you will end up building the kernel driver yourself.
I haven't had any problems with out-of-the-box Linux support for years, the only exception is my TUXEDO laptop.
This oddly puts TUXEDO as a Linux hardware vendor in a worse spot than non-Linux hardware vendors - at least from my point of view.

> Just as I was working on this article, some [news](https://www.phoronix.com/news/TUXEDO-Drivers-Taint-Patches) dropped regarding the kernel driver.
> It seems that they have at least recognized the problem and plan to work on upstreaming device support.

### Non-standard interfaces

TUXEDO's drivers offer so called performance profiles.
They are, as far as I can tell (because they are not documented in the code), similar to the performance profiles you can conveniently change in your DE (desktop environment), for example though the quick menu in GNOME.
They are not compatible though because on TUXEDO hardware, changing the power profile in your DE doesn't change anything, so you have to use their custom software.
I wonder why I have to install custom software for changing these profiles when my desktop environment already supports setting the performance profile conveniently.
This is also unfortunately something tuxedo-rs cannot address directly.

### The software stack

I usually don't care much in which programming language software is written, as long as it works.
However, it matters in the case of [TUXEDO's hardware control software](https://github.com/tuxedocomputers/tuxedo-control-center).
Choosing web technologies not only for the GUI, but also the underlying system service is more than questionable.
After all, the whole point of Electron is that makes it easy to port code to other operating systems.
Yet, the software is Linux-only.
The dependency on Node.js not only makes it painful to build the software yourself, it also makes it unlikely for maintainers of Linux distributions to package the software for their users.
Not to mention the increased RAM usage of the GUI caused by running essentially a whole webbrowser in the background.

## TUXEDO's Software Strategy: Problems for tuxedo-rs

The problems mentioned so far are mostly the reason why tuxedo-rs exists.
However, there are also several hidden problems that I have encountered during development of tuxedo-rs.

### The kernel driver

It turns out, the kernel driver not only lives out-of-tree, but it has some big problems with its interfaces to user space.
TUXEDO orders hardware components from several other vendors and for each vendor they use a separate driver with its own interface.
In tuxedo-rs, we have done a lot of work to built nice abstractions around two of the interfaces, but some of newer laptops aren't supported because they have an entirely different driver interface.
Unfortunately, there are at least three hardware vendors, thus at least three interfaces and each interface needs custom code from user space to make it work.
That's a lot of work and on top of that, on my hardware I can just test one of those interfaces.
Given that they are in control of their driver, I don't fully understand why they offload all the complexity to user space when they could have unified the interfaces of their different kernel drivers.

### Lacking generalization

The entire software stack of TUXEDO is tightly integrated, instead of working on a generic solution.
As said earlier, they could have an in-tree kernel driver and could support common interfaces.
If what's available doesn't work for them, they could work towards a new standardized solution that works for everyone.
That's at least what most other Linux hardware vendors are doing as far as I can tell.
I'm also not saying that TUXEDO doesn't work towards generic solutions, but so far, I haven't seen anything happen.

With tuxedo-rs, I want to pivot away from that concept.
It makes much more sense to build standardized components, such as the `org.freedesktop.UPower` DBUS service for example.
There could be one GUI application for all Linux devices that exposes the functionalities supported by the drivers.
TUXEDO would still have an edge here because they offer more control over the hardware, but everyone would profit from more generic hardware control.
In fact, there are already generic apps for setting CPU and GPU clock speeds for example which overlap with the features of TUXEDO's software.

### Linux limitations

There's one problem that does not only affect TUXEDO, but the Linux kernel in general.
I seems to me like the kernel developers are not really focused on building nice and efficient interfaces for user space, at least for desktop users.
For example, some newer TUXEDO laptops have control over individual LEDs on the keyboard.
Because each LED has its own file in `sysfs`, doing color animations quickly results in thousands of writes per second, which is crazily inefficient.
In my opinion, kernel developers should focus more on standardized interfaces that deal with controlling modern hardware from user space as this seems to be something that Windows has already solved much better.

### Lacking hardware access

With only two TUXEDO devices at home, I have very little options for testing TUXEDO hardware.
This is actually overall the biggest problem for me because I can hardly implement new features that my hardware doesn't support due to lack of testing.
Of course Rust gives me a lot of confidence, but dealing with the kernel interfaces without testing is still not realistic.

## Outlook

There are many users of tuxedo-rs and quite a few people have contributed so far.
It has been a very interesting experience for me and I'd like to continue maintaining the project.
However, I'm thinking about buying a new Laptop soon and it might not be TUXEDO hardware.
Not because their hardware is bad, but the software is just more tedious to work with than I would like.
And if I don't own any hardware, I cannot test the code at all and thus maintaining it doesn't make sense.
Maybe I'll merge smaller PRs from time to time, but I won't do anything apart from that.
That said, I'd be happy to onboard new maintainers that can help keeping the project alive.

## TL;DR

There are many reasons why TUXEDO hardware is great, but their software has some big issues.
Tuxedo-rs tries to address some of the problems, but some issues remain, which cannot be solved easily.
Therefore, I might not maintain the project in the future, but others are welcome to step up as maintainers.