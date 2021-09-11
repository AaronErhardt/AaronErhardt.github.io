# Worker template

```rust,no_run,noplayground
use gtk::prelude::{WidgetExt};
use relm4::*;

struct WorkerModel {

}

enum WorkerMsg {

}

impl Model for WorkerModel {
    type Msg = WorkerMsg;
    type Widgets = ();
    type Workers = ();
}

impl WorkerUpdate<AppModel> for WorkerModel {
    fn init_model(_parent_model: &AppModel) -> Self {
        WorkerModel {

        }
    }

    fn update(
        &mut self,
        msg: WorkerMsg,
        _Workers: &(),
        sender: Sender<WorkerMsg>,
        parent_sender: Sender<AppMsg>,
    ) {
        match msg {

        }
    }
}
```