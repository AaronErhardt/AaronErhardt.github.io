---
title: "A new Relm 4 you: announcing Relm4 v0.4!"
date: 2022-01-17T15:00:00+30:00
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

A new realm is here for you to explore! I'm happy to announce Relm4 v0.4, our biggest release so far!

> ## About Relm4
> 
> Relm4 is an idiomatic GUI library inspired by [Elm](https://elm-lang.org/) and based on [gtk4-rs](https://crates.io/crates/gtk4).
> 
> We believe that GUI development should be easy, productive and delightful.  
> The [gtk4-rs](https://crates.io/crates/gtk4) crate already provides everything you need to write modern, beautiful and cross-platform applications.
> Built on top of this foundation, Relm4 makes developing more idiomatic, simpler and faster and enables you to become productive in just a few hours.

# What's new?

Relm4 v0.4 brings a lot of new features and improvements. I'm excited to present to you the highlights of this release!

## Macros for everyone!

In recent Relm4 releases, the macros were tightly integrated into the library itself. This limitation is now removed! Most importantly, users of gtk4-rs will be able to use the `view!` macro by only pulling in `relm4-macros` as dependency without the rest of Relm4. This gives you even more freedom to choose how much of Relm4's features you want to use.

```rust
// Creating a box with a button inside.
relm4_macros::view! {
    vbox = gtk::Box {
        append = &gtk::Button {
            set_label: "Click me!",
            connect_clicked => |_| {
                println!("Hello world!");
            }
        },
    }
}

// You can simply use the vbox created in the macro.
let spacing = vbox.spacing();
```

And there's even more. The `view!` macro isn't limited to gtk4-rs. You can use it for regular Rust structures, too!

```rust
use std::process::Command;

let path = "/";

relm4_macros::view! {
    mut process = Command::new("ls") {
        args: ["-la"],
        current_dir = mut &String {
            push_str: path,
        },
        env: args!("HOME", "/home/relm4"),
    }
}

// Output of "ls -la" at "/"
dbg!(process.output());
```


## Micro components for flexibility at runtime

Regular components are relatively static, which gives you strong compile time guarantees, among other benefits. They cover most use cases, but they cannot be initialized or destructed manually at runtime.

This is where micro components have their strength. They are simpler variants of components that require a bit more manual work, with the advantage of being fully dynamic at runtime.

A code example can be found [here](https://github.com/AaronErhardt/Relm4/blob/main/relm4-examples/examples/micro_components.rs#L7).

## Type-save actions

[Actions](https://gtk-rs.org/gtk4-rs/git/book/actions.html) are used in GTK4 to simplify the handling of user interactions. Since you can always mix in as much pure gtk4-rs code into Relm4 applications as you want, actions were never a real problem. Yet, they lived outside of Rust's type checking.

With the help of some traits and macros, it's now quite convenient to define wrapper types for actions that enable a lot of compile time guarantees for actions. Even typos for action names are eliminated as a source of errors.

```rust
// Defines a new action group
new_action_group!(ActionGroup, "win");

// A new stateless action type that belongs to `ActionGroup`
new_stateless_action!(Action, ActionGroup, "action");

// A new action type with a `u8` as state and `u8` as target type
new_stateful_action!(U8Action, ActionGroup, "u8_action", u8, u8);

/* ... */

// This only works because `Action` is actually stateless
let action: RelmAction<Action> = RelmAction::new_stateless(move |_| {
    println!("Action called!");
});
```


## Menu macro

Often, actions are used for menus, where each entry is connected to an action. The new `menu!` macro allows you to create menus conveniently with all the benefits of type-safe actions.

```rust
// Create a `MenuModel` called `menu_model`
menu! {
    main_menu: {
        "action" => Action,
        "stateful action" => U8Action(1_u8),
        section! {
            "nested action" => Action,
            "nested stateful action" => U8Action(1_u8),
        },
    }
}
```

## More macro magic

The `widget` macro has been very helpful in creating readable and elegant UI definitions for regular components since the first release of Relm4.
Now, factories and micro components have received their own variants of the `widget` macro!

```rust
// A simple `FactoryPrototype` implementation
#[relm4::factory_prototype]
impl FactoryPrototype for Counter {
    type Factory = FactoryVec<Self>;
    type Widgets = FactoryWidgets;
    type View = gtk::Box;
    type Msg = AppMsg;

    view! {
        gtk::Button {
            set_label: watch!(&self.value.to_string()),
            connect_clicked(key) => move |_| {
                sender.send(AppMsg::Clicked(key)).unwrap();
            }
        }
    }

    fn position(&self, _index: &usize) {}
}
```

## Better support for Stack and TabView

Widgets such as `Stack` or `TabView` have methods that return a new widget, such as `StackPage`. The returned widget was previously inaccessibly in the `widget` macro and required manual code. This was addressed with [a new syntax](https://github.com/AaronErhardt/Relm4/blob/main/relm4-examples/libadwaita/examples/view-switcher.rs#L93).

## Cleaner dependencies

Relm4 now re-exports more crates and has more feature flags. This means you don't have to include `gtk4`, `libadwaita` or `relm4-macros` anymore, if you have the correct feature flags set.

```toml
relm4 = { version = "0.4", features = ["tokio-rt", "macros"] }
```

## Miscellaneous

+ Several book improvements
+ Reworked initialization process
+ Some trait adjustments
+ Many bug fixes, better error messages and other improvements

The full change log can be found [here](https://github.com/AaronErhardt/Relm4/blob/main/CHANGES.md#040---2022-1-16).

## Where to get started

+ ⬆️ **[Migration guide](https://aaronerhardt.github.io/relm4-book/book/0_2_to_0_4.html)**

+ ⭐ **[Repository](https://github.com/AaronErhardt/relm4)**
+ 📖 **[Book](https://aaronerhardt.github.io/relm4-book/book/)**
+ 📜 **[Rust documentation](https://aaronerhardt.github.io/docs/relm4/relm4/)**

> Note: As of writing this, the book isn't fully ported to v0.4, but the examples in the repository are always up to date.

## Special thanks

I highly appreciate feedback and contributions to Relm4 and thank those who helped me with this release:

+ [@euclio](https://github.com/euclio) who contributed many fixes and features
+ [@mskorkowski](https://github.com/mskorkowski) for his contributions and work on [relm4-store](https://github.com/mskorkowski/relm4-store)
+ [@tronta](https://github.com/tronta) who contributed a lot to the examples and book improvements
+ everyone else who gave feedback in the Matrix room or on GitHub
+ the whole gtk-rs team for providing awesome Rust bindings for GTK and always being helpful