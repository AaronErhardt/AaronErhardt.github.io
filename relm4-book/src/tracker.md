# Tracker

A tracker in this context just means a data type that's able to track changes to itself. For example if we increment the counter of the model we used for our first app the model could tell us later that the counter did change during the last update function.

Relm4 does not promote any implementation of a tracker you're free to use any implementation you like. You can even implement a tracker yourself. In this example however we'll use the tracker crate that provides a simple macro that implements a tracker for you automatically.

## The tracker crate

The `tracker::track` macro implements the following methods for your struct fields:

+ `get_#field_name()`  
  Get a immutable reference to your field

+ `get_mut_#field_name()`  
  Get a mutable reference to your field. Assumes the field will be modified and marks it as changed.

+ `set_#field_name(value)`  
  Get a mutable reference to your field. Marks the field as changed only if the new value isn't equal with the previous value.

+ `update_#field_name(fn)`  
  Update your mutable field with a function or closure. Assumes the field will be modified and marks it as changed.

To check for changes you can call `var_name.changed(StructName::field_name())` and it will return a bool indication whether the field was updated.

To reset all previous changes you can call `var_name.reset()`.

## Example

Let's have a look at a small example.

```rust
#[tracker::track]
struct Test {
    x: u8,
    y: u64,
}

fn main() {
    let mut t = Test {
        x: 0,
        y: 0,
        // the macro generates a new variable called
        // "tracker" that stores the changes
        tracker: 0,
    };

    t.set_x(42);
    // let's check whether the change was detected
    assert!(t.changed(Test::x()));

    // reset t so we don't track old changes
    t.reset();

    t.set_x(42);
    // same value so no change
    assert!(!t.changed(Test::x()));
}
```

> More information about the tracker crate can be found [here](https://github.com/AaronErhardt/Tracker)

So in short the `tracker::track` macro provides different getters and setters that will mark struct fields as changed. You also get a method that checks for changes and a method to reset the changes.

# Using trackers in Relm4 apps

Let's build a simple app that shows two random icons and allows the user to set each of them to a new random icon. As a bonus we want to show a fancy background color if both icons are the same.

> An almost identical example called "tracker" is available [here](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## The icons

Before we can select random icons we need to quickly implement a function that will return us random images.

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:icons }}
```

## The model

For our model we only need to store the two icon names and if both of them are identical.

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:model }}
```

The message type is also pretty simple: we just want to update the icons.

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:msg }}
```

There are a few notable things for the `AppUpdate` implementation.
First, we call `self.reset()` at the top of the update function body. This ensures that the tracker will be reset so we don't track old changes.

Also we use setters instead of assignments because we want to track these changes. Yet, you could still use the assignment operator if you want to apply changes without notifying the tracker.

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:app_update }}
```

## The widgets

Now we reached the interesting part of the code where we can actually make use of the tracker. Let's have a look at the complete widget macro:

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:widgets }}
```

The overall UI is pretty simple: A window that contains a box. This box has two boxes itself for showing the two icons and the two buttons to update those icons.

There's also something new. With the `pre_init()` and `post_init()` functions you can add custom code that will be run either before or after the code the widget macro generates for initialization. In our case we want to add custom CSS that sets the background color for elements with class name "identical".

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:post_init }}
```


### The track! macro

The `track!` macro is a simple macro that can be used inside the widget macro and allows us to pass a condition for updates and then the arguments. So the syntax looks like this:

```rust,no_run,noplayground
track!(bool_expression, argument, [further arguments])
```

Let's have a look at it's first appearance:

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:track1 }}
```

The macro expansion for the `track!` macro in the generated view function looks roughly like this:

```rust,no_run,noplayground
if model.changed(AppModel::identical()) {
    self.main_window.set_class_active("identical", model.identical);
}
```

That's all. It's pretty simple actually. We just use a condition that allows us to update our widgets only when needed.

The second `track!` macro looks very similar but only passes one argument:

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:track2 }}
```

> Since the `track!` macro parses expressions you can use the following syntax to debug your trackers:
> 
> `track!(bool_expression, { println!("Update widget"); argument })`

## The main function

There's one last thing to point out. When initializing our model, we need to initialize the `tracker` field as well. The value doesn't really matter because we call `reset()` in the udpate function anyway, but usually `0` is used.

```rust,no_run,noplayground
{{#include ../listings/tracker.rs:main }}
```

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:all }}
```