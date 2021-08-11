# Your first app

For our first app let's create something original: A simple counter app ;)

> An almost identical example called "simple_manual" is available [here](https://github.com/AaronErhardt/relm4/tree/main/relm4-examples) if you want to see the code in action.

## The basic structure 

Relm4 builds on the Elm programming model. This means there are three important data types you need to define:

+ The model that stores you application data, for example a counter
+ The message type that defines which messages can be sent to modify the model
+ The widgets type that stores the GTK widgets (UI elements)

Alright, let's have a look how this looks like for our counter app.

### The model

Our app only needs to store the state of a counter, so a simple `u8` will do the job for us.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:model }}
```

Because we want to initialize the counter with value 0 we can derive `Default` here.

### The message

Now we need to define what messages can be used to modify the model. The message could be represented by any data type but most often an `enum` is used. In our case we just want to be able to increment and decrement the counter.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:msg }}
```

### The widgets

The widgets `struct` stores the widgets we need to build our user interface. So for our app we could use a window with an increment button, a decrement button and a label to display the counter value. Additionally we need a box as a container to place our buttons and the label inside because a window can only have one child.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:widgets }}
```

## The Model trait

We have our data types in place, so now we can start implementing the model trait. This trait allows us to associate a model with other types to reduce the amount of generic parameters in other trait implementations.

There are three types we need to include:

+ Msg: what message type do we use to update the model?
+ Widgets: what widgets `struct` stores the widgets of our UI?
+ Components: which child components does our model use?

We don't care about components for now because we are just writing a simple app. Therefore we can use `()` as placeholder.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:model_trait }}
```

## The update loop

As a next step we want to make our app interactive. Relm4 has two important functions that update state and UI:

+ update: Receives a message and modifies the model
+ view: Receives the modified model and updates the UI accordingly

Before anything happens, a message must be sent through a channel. Theoretically anything can send messages but usually you send messages when a button is clicked or similar events occur. We will have a look how this works later.

![relm update loop](img/update_loop.svg)

> Data and widgets are separated from each other so that the update function doesn't interact with the widgets and the view function doesn't modify the model.

### The AppUpdate trait

Theory is nice, but let's see it in action.

Our update function is implemented in the `AppUpdate` trait:

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:app_update }}
```

> `wrapping_add()` and `wrapping_sub()` are like `+1` and `-1` but don't panic on overflows.

So what happens here? We just process the message and modify our counter accordingly.

Also we return `true` because we don't want to quit our application. If our app should close we can simply return `false` to shut down the application.

### The Widgets trait

Our last step is implementing the widgets trait. It provides methods to initialize the UI and to update the UI with the already mentioned view function.

Let's do this step by step. First we'll have a look at beginning of the trait `impl`.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:widgets_trait_start }}
```

You'll notice that

+ there are two generic parameters
+ a `Root` type

The two generic parameters are our model and the parent model. The parent model is only interesting for components which we will discuss later so again we can simply use `()` as placeholder.

The `Root` type is the root widget of the app. Components can choose this type freely but the main application must use a `gtk::ApplicationWindow`.

Next up, we want to initialize our UI.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:init_view }}
```

Again, what happens here? First we initialize each of our widgets. 

Then we connect them so that GTK4 knows how they are related to each other. The buttons and the label are added to the box and the box is added to the window.

Now the magic happens: we connect the "clicked" event for both buttons and send a message from the closures. We move a clone of our sender into the closures to send messages back to out update loop.

> The `send!(btn_sender, AppMsg::Increment)` macro simply expands to `btn_sender.clone().send(AppMsg::Increment).unwrap()`

Alright, now every time we click our buttons a message will be sent to update our counter!

Yet our UI will not update itself with our counter. To do this we just need to implement the view function:

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:view }}
```

Yep, that's it. We just need to update the label to represent the new counter value.

We're almost done. To complete the `Widgets` trait we just need to implement the `widget` method.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:root_widget }}
```

## Running the App

The last step is to run the app we just wrote. To do so, we just need to initialize our model and pass it into `RelmApp::new()`.

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:main }}
```

🎉 Congratulations! You just wrote your first app with Relm4! 🎉

### Conclusion

There are a few concepts in Relm4 that might look complex at first but are actually quite easy to understand and help you keeping your code structured. I hope this chapter made everything clear for you :)

As you have seen, initializing the UI was by far the largest part of our app with roughly one half of the total code. In the next chapter we will have a look at the relm4-macros crate that offers a macro that helps us to reduce the amount of code we need to implement the `Widgets` trait.

> As you might have noticed storing inc_button, dec_button and vbox in our widgets `struct` is not necessary because GTK will keep them alive automatically. Therefore we can remove them from `AppWidgets` to avoid compiler warnings.

## The complete code

Let's review our code in one piece one more time to see how all these parts work together:

```rust,no_run,noplayground
{{#include ../listings/simple_manual.rs:all }}
```