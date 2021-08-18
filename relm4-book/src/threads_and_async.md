# Threads

In the previous chapter, we've introduced workers. One of the most common use case for workers is to run long computations or IO-bound operations on a different thread. So how do we do this?

## Running a component on a different thread

You might remember this section of code from the example application in the components chapter.

```rust,no_run,noplayground
{{#include ../listings/components.rs:components_impl }}
```

In order to run the dialog component on a new thread, we just need to change one line:

```rust,no_run,noplayground
impl Components<AppModel> for AppComponents {
    fn init_components(
        parent_model: &AppModel,
        parent_widgets: &AppWidgets,
        parent_sender: Sender<AppMsg>,
    ) -> Self {
        AppComponents {
            header: RelmComponent::new(parent_model, parent_widgets, parent_sender.clone()),
            dialog: RelmComponent::with_new_thread(parent_model, parent_widgets, parent_sender),
        }
    }
}
```

It's hard to spot the difference. Instead of `RelmComponent::new` we used `RelmComponent::with_new_thread`. The same works for worker. `RelmWorker::new` runs the worker on the same thread and `RelmWorker::with_new_thread` spawns a new thread for the worker.

There's one problem for the components, though. Components have widgets that in the case of GTK4 are neither `Send` nor `Sync`. That means we can't run the view function from a different thread, but only the update function that just operates on the model. But then, how does this work? Well, Relm4 sends the model from a new thread that handles the update function to the main thread that then handles the view function and back to the new thread again. This is not optimal regarding performance and therefore only recommended if you don't send a lot of messages to the component. Alternatively, you can always do the heavy work in a worker because workers don't have this problem.

## Async

Async update functions currently exclusive for workers currently (if you need async components please open an issue). If you enable the tokio-rt feature, you can use an `AsyncRelmWorker` type that uses an async update function. Apart from that, they are just like normal workers that run in a new thread. The ["tokio" example](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/tokio.rs) shows how this can be used with for async HTTP requests.

## The message queue problem

Because workers tend to take a lot of time during the update function you should make sure to not bombard them with messages. Imagine you have a button in your application that allows the user to update a web page. If the user presses the button, a new request is sent and then the worker responds with a message. If the button can be clicked and a message is sent for each click while the worker fetches the web page you could quickly have a lot of unprocessed messages in the queue of your worker. To avoid this, make sure to only send the message once and wait until the worker is finished.

## Multiple threads and async without workers

One reason you always get a new sender passed into your update function is that you can spawn a new thread and move a cloned sender into it. This can sometimes be more flexible than defining a worker. You can simply use `std::thread::spawn` for this and even spawn any async runtime you want from that thread.

For example you could do this in your update function:

```rust,no_run,noplayground
std::thread::spawn(move || {
    send_request();
    send!(sender, AppMsg::RequestComplete);
});
```

### Async inside the main event loop

GTK uses an event loop from glib to handle asynchronous events. In fact the senders we've been using all the time are just channels on that event loop. This event loop also allows us to execute futures. Relm4 provides a `spawn_future` function to do exactly that. The only drawback of this is that most crates relying on a tokio runtime won't work and that the future is run on the main thread. The ["future" example](https://github.com/AaronErhardt/relm4/blob/main/relm4-examples/examples/future.rs) shows how this can be used.