---
title: "Announcing tuxedo-rs"
date: 2023-03-15
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

I'm excited to announce [tuxedo-rs](https://github.com/AaronErhardt/tuxedo-rs), a collection of Rust crates for interacting with hardware from TUXEDO Computers.

## Some background

[TUXEDO Computers](https://www.tuxedocomputers.com/) is a PC and notebook vendor from Germany that sells hardware optimized for Linux.

*TUXEDO Computers are customizable Linux notebooks and Desktop PCs optimized in the first place to run with Ubuntu-based Linux operating systems.*
*Literally Linux hardware in a tailor-made suit.*

I think that's pretty neat! 
For me, having a small local hardware vendor that focuses on Linux is basically all I could wish for.

### The start of a journey

Over one year ago, I bought my first TUXEDO notebook, installed my favorite Linux distribution and everything worked great out of the box.
Eager to unlock the full potential of my new hardware, I installed the TUXEDO Control Center shortly afterwards.
The [TUXEDO Control Center](https://www.tuxedocomputers.com/en/TUXEDO-Control-Center.tuxedo#) (short TCC) is a *comprehensive tool for letting you control and monitor all safety-, performance-, energy- and comfort functions of your TUXEDO laptop*.

Unfortunately, I wasn't quite happy with my experience overall.
The installation of the TCC already required more manual steps and had more gotchas than I would have liked.
I was genuinely surprised how much extra effort I had to put in for getting TCC to run on my system.
To be fair, I chose not to use TUXEDO OS because it doesn't suit my needs and expected a few extra steps, but I still was a little bit disappointed.

Surely, better documentation would have helped me a bit, but the far bigger issues stem from the underlying decisions and the technology stack.
Using a mix of C++ and JavaScript for TCC and its underlying system service not only makes the installation more complex but also makes it a hard for Linux distribution to package TCC.
Even more odd, however, is the fact that a Linux hardware vendor (and KDE sponsor) doesn't even ship a native GUI for their applications.
TCC's web-UI looks completely out of place and consumes much more resources than necessary.

Not happy with the whole situation, I started thinking about a native replacement for TCC.
A solution that would be intuitive, less resource hungry, easy to install and simple to package.

## Introducing Tuxedo-rs

<div align="center">
    <img src="./tuxedo-rs-mascots-dark.png" style="width: 10em; max-width: 100%;"/>
    <i>Tux and Ferris are a great team!</i>
</div>

### Level 1 - Hardware abstraction

TUXEDO Computer's kernel driver exposes two interfaces, sysfs for LED control and ioctl for almost everything else.
First, I started exploring the sysfs interface with a small shell script.
Shortly afterwards, I wrote the `tuxedo_sysfs` crate as a proper abstraction over the sysfs interface.
Until recently, TUXEDO used a non-standard interface for controlling the RGB lights of supported keyboards.
Yet with the latest version of the kernel driver, TUXEDO moved their interface to `/sys/class/led` which makes it easy for `tuxedo_sysfs` to pick up LED devices from other manufacturers as well.

The ioctl interface turned out to be a bit more challenging.
TUXEDO Computers uses components from different manufacturer which have different interfaces in the ioctl driver.
Fortunately, the `tuxedo_ioctl` crate has a hardware abstraction layer that lets you forget those differences and makes it easy to figure out device capabilities.
Depending on the manufacturer, different traits are implemented that represent a subset of all available features.
Once you connect to the interface, you either get a trait object with the supported functionality or [`None`](https://doc.rust-lang.org/std/option/enum.Option.html#variant.None) if the device doesn't support a certain feature.
Basically, hardware support is perfectly matched by Rust's type system.

### Level 2 - The system service

With the first layer completed, the next step was to build a system service that's able to manage the hardware and different configurations.
The resulting `tailord` crate takes care of all the heavy lifting and exposes a simple DBUS interface.

Similar to the TCC, `tailord` is heavily focused around profiles.
First, you can add and edit different fan curves and RGB animations.
Then, you use profiles to assign the various configurations to your hardware.
By selecting another profile, you can easily switch between different configurations.

Compared to the TCC, `tailord` has some additional features, but is also missing some features.
We're currently working on closing the gap, but `tailord` should already offer more than enough for most users.
Due to using Rust and better control algorithms, `tailord` is able to greatly outperform TCC's `tccd` service both in CPU and RAM usage.
To be fair, `tccd` already does quite ok, but `tailord` is at least 5 times more efficient in both categories.

### Level 3 - The client library

Using raw DBUS interfaces for interacting with `tailord` is not so much fun.
Luckily, `tailor_client` provides a neat asynchronous API as an abstraction layer for communication with `tailord`.
Connecting to `tailord`, serializing and deserializing of messages and error handling is all handled by this crate.
By sharing common code with `tailord` through `tailor_api`, it is guaranteed that server and client can communicate seamlessly.

### Level 4 - The GUI

Built on the `tailor_client` crate, `tailor_gui` offers a native and intuitive UI for managing your hardware.
On three pages, you can modify your profiles, RGB animations and fan curves.
On every step, the UI will give you feedback instantaneously to make it easier to figure out which RGB value or fan speed is optional for you.

The following video shows the features of the GUI, but note that I made this video already a couple of weeks ago and more features have been added since.

<video controls style="width: 100%;">
    <source src="./tuxedo-rs.mp4" type="video/mp4">
    Your browser does not support the video tag.
</video> 

### Overview

As you can see, Tuxedo-rs is not just a replacement for TCC, but a collection of crates that offer abstractions at several levels.
No matter whether you want to write a small utility, an alternative system service or build your own UI, tuxedo-rs has you covered.
If you'd like to have an overview of all crates and how they interact with each other and the hardware, check out [this graphic on our repository](https://github.com/AaronErhardt/tuxedo-rs#structure).

## Why Rust?

Of course, Rust is not a magic programming language that solves all problems and should replace all languages.
Yet, especially for the scope of tuxedo-rs, it has proven to be a great choice.
It is well known that Rust has outstanding performance, great tooling and tends to result in very robust programs.
I think this has been covered well enough already, so I'd like to focus on some aspects that helped with tuxedo-rs in particular.

An interesting aspect of tuxedo-rs is that you will probably find equally suited or even better languages for certain parts of the project.
Rust might not be a superior language for low-level kernel interfaces, system services or GUIs.
Yet, it manages to be a great choice for all those tasks and makes it possible to write everything in just one language.
In general, Rust simply accumulates a lot of great features that you would miss in other languages.

In particular, async support turned out to be an essential feature.
Managing multiple DBUS connections, running fan control algorithms and showing RGB animations on a single thread could hardly be easier.
Together with channels, asynchronous programming in Rust makes it quite easy to write robust concurrent applications. 

One thing I really appreciate as well is the strong type system.
Tuxedo-rs already has quite a lot of crates and during the development, it was not uncommon to change something in a lower level to be able to extend the functionality somewhere else.
After going through a couple of error messages, the program would be fully functional again with no accidental bugs added in the process.
The explicit type system is also very important for robust error handling and makes easy to write abstractions that cannot be used incorrectly.
I think especially for system services which run with elevated permissions, correctness and safety cannot be valued enough.

On top of that, Rust also has a great ecosystem.
[`nix`](https://crates.io/crates/nix) offers great abstractions over ioctl and [`tokio_uring`](https://crates.io/crates/tokio_uring) made it very simple to access the sysfs interfaces asynchronously through [`io_uring`](https://kernel.dk/io_uring.pdf).
For DBUS interfaces [`zbus`](https://crates.io/crates/zbus) is an outstanding crate for both servers and clients.
And finally, [`Relm4`](https://crates.io/crates/relm4) makes it easy to write native GUI applications and offers great support for asynchronous operations.

## How can I use tuxedo-rs?

Tuxedo-rs has just been announced with this post, so unsurprisingly there won't be any packages available for your favorite Linux distribution yet.
Tailor GUI will hopefully be published soon on flathub and we would love to work together with package maintainers to package the individual components of tuxedo-rs.

For now, building from source is still necessary.
Fortunately, the build process is very simple and only requires [meson](https://mesonbuild.com/) and [Rust](https://rustup.rs).
You can find detailed instructions [here](https://github.com/AaronErhardt/tuxedo-rs#installation).

### What if I don't own TUXEDO hardware?

Tuxedo-rs is a purely community-driven project.
It is not developed nor supported by TUXEDO Computers.
This means that tuxedo-rs is free to implement support for other vendors as well.
Technically, some features like RGB animations should already work with all RGB devices with a decent Linux driver.
For more advanced features however, we need contributions from the community to add more devices.

## Future plans

I'm already quite happy with tuxedo-rs, but there are still a lot of features to work on.
Eventually, we might even upstream a Kernel driver as replacement for the modules offered by TUXEDO Computers which can't be merged into the Kernel due to an incompatible license.

Most importantly however, tuxedo-rs needs more testing due to the limited hardware available to the contributors.
Let us know if you want to add new features or experienced a problem.
Contributions are always highly appreciated!

## Credit

I want to thank TUXEDO Computers for offering great Linux hardware and publishing the source code of their software.
Even though the documentation is mostly lacking, reading source code is still far better than reverse engineering and helped a lot during the development.

Especially, I want to thank [Matteo Bigoi](https://github.com/crisidev) for contributing support for hardware from Uniwill (which is used in a lot of devices from TUXEDO Computers) and several other features to tuxedo-rs!
